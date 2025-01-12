use std::cell::Cell;
use std::sync::{Arc, Mutex};
use std::thread;

pub mod test;

fn main() {
    test::tester();
    let arc_dumb_gaurd = Arc::new(Mutex::new(10));

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let arc_dumb_gaurd = Arc::clone(&arc_dumb_gaurd);
        let handle = thread::spawn(move || {
            println!("{:?}", arc_dumb_gaurd);
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
