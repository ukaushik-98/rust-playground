use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn foo<'a>() -> Mutex<Vec<&'a str>> {
    let x = vec!["hello"];
    let mx = Mutex::new(x);
    {
        // the same principle from line 19 is occurring here
        let mut y = mx.lock().unwrap();
        y.push("world");
        // this is legal because MutexGuard is Sync i.e. &MutexGuard is Send
        foo_send(&y);
        // this is not legal because MutexGuard is Sync but !Send
        // foo_send(y);
    }
    mx
}

pub fn foo_send<T: Send>(_v: T) {}

pub fn foo_thread() {
    let x = Mutex::new(vec!["hello"]);
    let mx = x.lock().unwrap();
    let a = Arc::new(&mx);
    let ac = Arc::clone(&a);
    foo_send(ac);
    // let _ = thread::spawn(move || {
    //     let mac = ac;
    // })
    // .join();
}

pub fn foo2() -> Vec<&'static str> {
    vec!["hello"]
}

// ownership allows you to declare if something has mutable access
pub fn foo3() {
    let mut x = foo2();
    x.push("world");
}

// T must be returned
pub fn foo4<'a, T>(x: &'a T) -> Vec<&'a T> {
    vec![x]
}

// this won't work because T must be returned and we're trying to return a string
pub fn foo5<'a, T>(_x: &'a T) -> Vec<&'a str> {
    let y = "hello";
    vec![y]
}

// this won't work because T must be returned and we're trying to return a string
// pub fn foo6<'a, T>(_x: &'a T) -> Vec<&'a T> {
//     let y = "hello";
//     vec![y] // mismatched types expected reference `&T` found reference `&str`
// }

// same thing here - this won't work because the expected type T is not necessarly a string
// pub fn foo7<'a, T>() -> Vec<&'a T> {
//     let x = String::new();
//     let y: &'a String = &x;
//     vec![y] // mismatched types expected reference `&T` found reference `&str`
// }

#[test]
fn foo_test() {
    let f = foo();
    let f = f.lock().unwrap();
    assert_eq!(vec!["hello", "world"], *f);
}

fn garbage<'a>(hello: &'a String) {
    let y: &'a String = &hello;
    // owned_runner().await;
    let x: Vec<&String> = vec![y];
    println!("x: {:?}", x);
}
