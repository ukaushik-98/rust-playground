use std::sync::Mutex;

pub fn foo<'a>() -> Mutex<Vec<&'a str>> {
    let x = vec!["hello"];
    let mx = Mutex::new(x);
    {
        // the same principle from line 19 is occurring here
        let mut y = mx.lock().unwrap();

        y.push("world");
    }
    mx
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
//     vec![y]
// }

// same thing here - this won't work because the expected type T is not necessarly a string
// pub fn foo4<'a, T>() -> Vec<&'a T> {
//     let x = String::new();
//     let y: &'a String = &x;
//     vec![y]
// }

#[test]
fn foo_test() {
    let f = foo();
    let f = f.lock().unwrap();
    assert_eq!(vec!["hello", ""], *f);
}
