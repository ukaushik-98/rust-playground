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
