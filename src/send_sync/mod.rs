mod sync;
use std::cell::Cell;

fn foo<'a>(x: &'a Vec<i32>) -> Cell<&'a Vec<i32>> {
    let d: &'a Vec<i32> = &x;
    let c: Cell<&'a Vec<i32>> = Cell::new(d);
    let a = d;
    c
}

fn foo2<'a>() /*  -> Cell<&Vec<i32>> */
{
    let x: Vec<i32> = vec![5, 5, 5];
    let d: &Vec<i32> = &x;
    let c: Cell<&Vec<i32>> = Cell::new(d);
}

fn bar() {
    let x: Vec<i32> = vec![5, 5, 5];
    let y = foo(&x);
    let b = y;
}
