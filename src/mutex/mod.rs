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

// pub fn foo4<'a, T>() -> Vec<&'static T> {
//     vec!["hello"]
// }

#[test]
fn foo_test() {
    let f = foo();
    let f = f.lock().unwrap();
    assert_eq!(vec!["hello", ""], *f);
}
