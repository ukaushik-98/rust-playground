use std::sync::Arc;

fn foo() {
    let a = vec![2];
    let x = Arc::new(a);
    let y = Arc::clone(&x);
}

fn foo2() {
    let a: &str = "hello";
    let x = Arc::new(a);
    let y = Arc::clone(&x);
}
