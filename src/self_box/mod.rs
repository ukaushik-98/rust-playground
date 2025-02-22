fn box_vec(x: &mut Vec<&str>) {
    x.push("world");
}

fn boxing() {
    let mut x = Box::new(vec!["hello"]);
    let y = &mut *x;
    // x.push("value");
    box_vec(y);
}

fn boxing2() {
    let mut v = vec!["hello"];
    let mut x = Box::new(&v);
    // v.push("value");
    println!("{:?}", x);
}
