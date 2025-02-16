fn base<'a>(x: &'a i32, y: &'a mut i32) -> &'a mut i32 {
    if *x > 10 {
        *y = 1;
    } else if *x > 5 {
        *y *= 2;
    }
    y
}

// We used the fact that &mut u32 can't be aliased to prove that writes to *output can't possibly affect *input.
// This lets us cache *input in a register, eliminating a read.
// By caching this read, we knew that the write in the > 10 branch couldn't affect whether we take the > 5 branch,
// allowing us to also eliminate a read-modify-write (doubling *output) when *input > 10.
fn base_opt<'a>(x: &'a i32, y: &'a mut i32) -> &'a mut i32 {
    // opt deref a single time
    let cx = *x;
    if cx > 10 {
        *y = 2;
    } else if cx > 5 {
        *y *= 2;
    }
    y
}

fn alias() {}

#[test]
fn base_test() {
    let x = 5;
    let mut y = 6;
    // it isnt possible in rust to have a &str or &mut str
    // so the base_opt
    base_opt(&x, &mut y);
}
