use std::cell::{Cell, RefCell};
mod g2;
mod garbage_cell;
mod garbage_ref;
mod garbage_ref_cell;
mod garbage_ref_mut;

fn foo() {
    let mut x = vec![1, 2, 3];
    let y = &mut x;
    // ❌ can't borrow x again as mut after mut borrow
    // let z = &mut x;
    y.push(4);
    x.push(5);
}

fn foo2() {
    let mut x = vec![1, 2, 3];
    let y = &x;
    // ❌ can't exclusively borrow x again as mut after shared borrow
    // let z = &mut x;
    // ❌  can't borrow x as mut since immutable pointer exists
    // x.push(5);
    println!("{:?}", y)
}

fn foo3() {
    let mut x = vec![1, 2, 3];
    let mut y = Cell::new(&mut x);
    // ❌ can't exclusively borrow x again after placing mut ref into cell
    // let a = &mut x;
    let z = y.get_mut();
    z.push(4);
    x.push(5);
}

fn foo4() {
    let mut x = vec![1, 2, 3];
    let mut y = Cell::new(&mut x);

    // ❌ can't exclusively borrow x again after placing mut ref into cell
    // let a = &mut x;
    let z = y.get_mut();

    // ❌ can't exclusively borrow as mut again
    // x.push(5);
    z.push(4);
}

fn foo5(_x: &mut Vec<i32>) {
    let mut x = 5;
    let mut y = Cell::new(x);
    // get only available when T in Cell is copy
    let z = y.get();
}

fn foo6() -> Vec<i32> {
    let mut x = vec![1, 2, 3];
    let y = Cell::new(&mut x);
    bar(y);
    x.push(5);
    x
}

fn bar(mut a: Cell<&mut Vec<i32>>) {
    a.get_mut().push(4);
}

fn foo7() {
    let x = vec![1, 2, 3];
    let y = &x;
    // can't move x because a pointer exists
    // if line 70 is commented out though, it will work due to covariance
    // let z = Cell::new(x);
    println!("{:?}", y);
}

fn foo8() {
    let mut x = vec![1, 2, 3];
    let y = &mut x;
    let z = Cell::new(y);
    // cant get on mut ref because it doesnt implement copy
    // let a = z.get();
    // println!("{:?}", y);
}

fn foo9() {
    let mut x = vec![1, 2, 3];
    let y = &x;
    let z = Cell::new(y);
    // cant borrow mut ref because immutable ref is given
    // let a = &mut x;
    println!("{:?}", z);
}

fn foo10() {
    let mut x = vec![1, 2, 3];
    let y = &x;
    let z = Cell::new(y);
    // cant borrow mut ref because immutable ref is given
    // x.push(5);
    println!("{:?}", z);
}

fn foo11() {
    let mut x = vec![1, 2, 3];
    let y = &mut x;
    let mut z = Cell::new(&mut x);
    // cant borrow mut ref because mut ref is given
    // x.push(5);
    drop(z);
}

fn foo12() {
    let mut x = Cell::new(vec![1]);
    let y = &x.get_mut()[0];
    // x.set(vec![]);
    // let y2 = &x.get_mut()[0];
    // drop(x);
    // let y3 = x.get_mut()[0];
    println!("{}", y);
    // println!("{}", y3);
}

struct Dumb<T> {
    inner: T,
}

fn garbo<'a>() {
    let mut x = String::from("");
    let y = &x;
    // x = String::from("hello");
    println!("{}", y);
}

fn garbo2<'a>() {
    let mut x = Dumb { inner: "" };
    let y = &x.inner;
    // cannot update because of borrow
    // x.inner = "hello";
    println!("{}", y);
}

fn bar2() {
    let mut x = vec![1];
    let y = &x;
    x.push(4);
}

fn bar3() {
    let a = vec![1];
    let x = RefCell::new(a);
    let mut z = x.borrow_mut();
    z.push(2);
    let y = x.borrow();
    println!("{:?}", x);
}

#[test]
fn foo6_test() {
    assert_eq!(foo6(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn bar3_test() {
    bar3()
}
