use std::{
    cell::Cell,
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

fn foo2<'a: 'static>(x: Arc<&'a Vec<i32>>) {
    let mut thread_handles = Vec::new();
    for _ in [..10] {
        let ref_x = x.clone();
        let handle = thread::spawn(move || {
            println!("{:?}", ref_x);
        });
        thread_handles.push(handle);
    }
}

fn foo() {
    let mut x = vec![1, 2, 3];
    let y: &mut Vec<i32> = &mut x;
    let x = Arc::new(x);
    let mut thread_handles = Vec::new();
    for _ in [..10] {
        let ref_x = Arc::clone(&x);
        let handle = thread::spawn(move || {
            println!("{:?}", ref_x);
        });
        thread_handles.push(handle);
    }
}

fn runner() {
    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..5 {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            let mut num = data_clone.lock().unwrap();
            *num += 1;
            println!("Incremented value: {}", *num);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final value after all threads have finished modifying the value.
    println!("Final value: {}", *data.lock().unwrap());
}

fn bar<T>(x: T)
where
    T: Sync,
{
    drop(x);
}

fn bar2() {
    let mut x = vec![1, 2, 3];
    bar(&x);
}

fn foo3<'a, 'b, T>(x: &'a Vec<&'a str>, y: &'b Vec<&'b str>)
where
    'a: 'b,
{
    let mut z = &x;
    z = &y;
}

/// The following is an example of how wrapping a type T in a mutex can turn it into Send + Sync despite the T itself only implementing Send
/// In order for a type to be send to another thread it must be Send + 'static, so these type still mantains it's send trait even after wrapping it a mutex.
/// However, we can coerce the type into Sync by the nature of the Mutex trait implementations as seen in the following examples
fn foo4() {
    // Cell = Send + !Sync
    let x = Cell::new(5);

    // Mutex<T> = Send + Sync where T is Send
    // Mutex<Cell> = Send + Sync where Cell is Send
    // Cell is Send ∴ Mutex<Cell> is Send and Sync
    let mx = Mutex::new(x);

    // Arc<T> = Send + Sync where T is Send + Sync
    // Mutex<Cell> = Send + Sync where Cell is Send ∴ Arc<Mutex<Cell>> = Send + Sync where Cell is Send
    let arc_mx = Arc::new(mx);
    let arc_mx_clone = Arc::clone(&arc_mx);

    let _ = thread::spawn(move || {
        let y = arc_mx_clone.lock();
    })
    .join();
}

fn foo5<T: Send + Sync>(a: Arc<Mutex<T>>) {}

fn foo6<T: Send>(a: Arc<Mutex<T>>) {}

fn foo7<T: Sync>(a: Arc<Mutex<T>>) {}

fn foo8() {
    let x = Cell::new(5);
    let mx = Mutex::new(x);
    let arc_mx = Arc::new(mx);
    // Fails to compile since Cell = Send + !Sync but the bounds on foo5 require Send + Sync
    // foo5(arc_mx);

    // For futher proof that the sync req is the problem, foo7 bound is lowered to JUST Sync and it still won't compile
    // foo7(arc_mx);

    // Compiles just fine since the bound is only Send
    foo6(arc_mx);
}
