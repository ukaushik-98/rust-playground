use std::cell::Cell;

fn foo() {
    let mut x = vec![1, 2, 3];
    let mut y = Cell::new(&mut x);
    let z = y.get_mut();
    z.push(4);
    x.push(5);
}
