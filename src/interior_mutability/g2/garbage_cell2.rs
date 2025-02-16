use std::cell::{Cell, UnsafeCell};

pub struct GCell2<T> {
    value: UnsafeCell<T>,
}

impl<T> GCell2<T> {
    pub fn new(value: T) -> GCell2<T> {
        GCell2 {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    fn bad_get(&self) -> &T {
        // Safety: THIS IS BAD
        // If we give out a reference to the value within Cell it's possible to change it under the hood
        // Think about normal references - you can't mutate once the ref has been created.
        // See test_2 for a concrete example
        //
        // For this use case, see test_1 for this bad behavior in action
        unsafe { &*self.value.get() }
    }
}

#[test]
fn test_1() {
    let mut a = vec!["hello"];
    let ar = &mut a;
    let x = GCell2::new(ar);
    let y = x.bad_get();

    // cannot borrow `a` as mutable more than once at a time second mutable borrow occurs here
    // a.push("value");

    // borrow of moved value: `ar` value borrowed here after move
    // println!("{:?}", ar);

    // x.set("world");
    // assert_eq!(y, "hello");
    // println!("{:?}", ar);
}

#[test]
fn test_2() {
    let x = "hello";
    let y = &x;
    // cannot assign to `x` because it is borrowed
    // `x` is assigned to here but it was already borrowed
    // x = "world";
    assert_eq!(*y, "hello");
}

#[test]
fn test_3() {
    let x = String::from("hello");
    let z = GCell2::new(x);
    let y = z.bad_get();
    z.set(String::from("world"));
    assert_eq!(*y, String::from("hello"));
}

#[test]
fn test_4() {
    let mut x = vec!["hello"];
    let y = &x[0];
    // x[0] = "world";
    println!("{:?}", y);
}

#[test]
fn test_5() {
    let x = GCell2::new(vec![1, 2, 3]);
    let first = &x.bad_get()[0];

    // BAD behavior - first should have been 1 but it changes under the hood
    // e.g. 1702064993
    x.set(vec![]);
    assert_eq!(*first, 1)
}
