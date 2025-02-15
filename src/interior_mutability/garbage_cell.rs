use std::cell::UnsafeCell;

pub struct GarbageCell<T> {
    value: UnsafeCell<T>,
}

impl<T> GarbageCell<T> {
    fn new(value: T) -> Self {
        GarbageCell {
            value: UnsafeCell::new(value),
        }
    }

    fn set(&self, value: T) {
        unsafe { *self.value.get() = value }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    fn bad_get(&self) -> &T {
        unsafe { &*self.value.get() }
    }
}

#[test]
fn bad() {
    let a = vec!["hello"];
    let x = GarbageCell::new(a);
    let y = &x.bad_get()[0].chars().nth(0).unwrap();
    x.set(vec![]);
    assert_eq!(*y, "hello".chars().nth(0).unwrap());
    // x.set(String::from("world"));
}
