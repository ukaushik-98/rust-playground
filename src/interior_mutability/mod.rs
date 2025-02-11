use std::cell::Cell;

fn foo() {
    let mut x = vec![1, 2, 3];
    let y = &mut x;
    // ✅ can't borrow x again as mut after mut borrow
    // let z = &mut x;
    y.push(4);
    x.push(5);
}

fn foo2() {
    let mut x = vec![1, 2, 3];
    let mut y = Cell::new(&mut x);
    // ✅ can't borrow x again after placing mut ref into cell
    // let a = &mut x;
    let z = y.get_mut();
    z.push(4);
    x.push(5);
}

fn foo3() {
    let mut x = vec![1, 2, 3];
    let y = &x;
    // ✅ can't borrow x again as mut after mut borrow
    // let z = &mut x;
    //  can't borrow x as mut since immutable pointer exists
    // x.push(5);
    println!("{:?}", y)
}
