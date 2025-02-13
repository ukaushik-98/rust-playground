use std::cell::Cell;

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
    // ❌ can't borrow x again as mut after mut borrow
    // let z = &mut x;
    //  can't borrow x as mut since immutable pointer exists
    // x.push(5);
    println!("{:?}", y)
}

fn foo3() {
    let mut x = vec![1, 2, 3];
    let mut y = Cell::new(&mut x);
    // ❌ can't borrow x again after placing mut ref into cell
    // let a = &mut x;
    let z = y.get_mut();
    z.push(4);
    x.push(5);
}

fn foo4() {
    let mut x = vec![1, 2, 3];
    let mut y = Cell::new(&mut x);

    // ❌ can't borrow x again after placing mut ref into cell
    // let a = &mut x;
    let z = y.get_mut();

    // ❌ can't borrow as mut again
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

#[test]
fn foo6_test() {
    assert_eq!(foo6(), vec![1, 2, 3, 4, 5]);
}
