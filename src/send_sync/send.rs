use std::{
    cell::Cell,
    sync::{Arc, Mutex},
    thread,
};

/// A somewhat surprising consequence of the definition is that &mut T is Sync (if T is Sync) even though it seems like that might provide unsynchronized mutation.
/// The trick is that a mutable reference behind a shared reference (that is, & &mut T) becomes read-only, as if it were a & &T.
/// Hence there is no risk of a data race.

/// A shorter overview of how Sync and Send relate to referencing:

/// &T is Send if and only if T is Sync
/// &mut T is Send if and only if T is Send
/// &T and &mut T are Sync if and only if T is Sync

fn foo_sync<T: Sync>(a: T) {}

fn foo_send<T: Send>(a: T) {}

fn foo_mut(a: &mut &mut Vec<&str>) {
    a.push("World");
}

fn foo2() {
    let mut x = vec!["helo"];
    // let mut x = Cell::new(5);
    // let mut x = &mut x;
    foo_send(&mut x);
    foo_sync(&mut x);
    // foo_mut(&mut x);
    x.push("check");

    // arc doesnt give a mutable type, i.e. it doesn't implement DerefMut
    // instead arc implements Deref and provides a immutable reference
    // and this makes sense! we're trying to send a mutable owned object to multiple
    // threads here and this isn't sound behavior.
    //
    // in order to actually get at the value inside, we need the ability to get mutable
    // access via an immutable pointer, i.e. an interior mutability type with sync capaiblity
    // This means that the guard must implement DerefMut: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
    // Trait Flow: Arc -> deref -> mutex -> mutex.lock -> deref mut -> push
    //
    // Why do types that do implement deref mut like Box or Vec not work here?
    // Arc only implements deref so it only gives our immutable reference.
    // Push, however, requires a mutable reference and there's no way for us to get a mutable Box or Vec
    // just from Arc.
    // Trait Flow: Arc -> deref -> &Vec != &mut Vec required by push

    let ax = Arc::new(x);
    // let ax = Arc::new(Mutex::new(x));
    let mut axc = Arc::clone(&ax);
    // let axc = Arc::clone(&ax);
    thread::spawn(move || {
        let axc = Arc::get_mut(&mut axc).unwrap();
        axc.push("value");
        // axc.lock().unwrap().push("value");
    });
}

#[test]
fn foo2_test() {
    foo2()
}
