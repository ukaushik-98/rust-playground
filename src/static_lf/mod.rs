// This function is a very simple test to see what qualifies as a 'static and what doesnt.
// Here we're bounding the lifetime of T with 'static as a test to confirm that a type conforms to 'static.
//
// A common misconception here is that 'static means that x must be created at compile time but what it really means is either
//  - &'static where & is valid across 'static. This implies that the T itself must live as long or longer that the bound. i.e. T is also 'static
//  - an owned type! After all, as long as a type is owned by the closure, it always have guranteed access and is therefore complies by the rule.
fn foo<T: 'static>(x: T) {}

fn bar() {
    let x = vec!["hello"];
    foo(x); // ✅ passing an owned type is completely valid!
    /// foo(&x); // ❌ this is invalid since x is the T for the &T and does not live long enough
    let y: &'static str = "hello";
    foo(y); // ✅ again, since y has a lifetime of 'static, it's valid to pass this into foo

    let z = vec!["hello"];
    let z: &'static mut Vec<&str> = Box::leak(Box::new(z));
    foo(z); // ✅ this time, we leverage box leak to give use back a lifetime of any, which can therefore be 'static
}
