fn test<'a>(hello: &mut Box<&'a str>, garbagio: &'a str) -> Box<&'a str> {
    std::mem::replace(hello, Box::new(garbagio))
}

fn new_tester<'b>() {
    let hello: Box<&'static str> = Box::new("hello");

    let mut world: Box<&'b str>;
    world = hello;
}

pub(crate) fn tester<'a>() {
    let mut hello: Box<&'static str> = Box::new("hello");
    let hello2: &'a str;
    {
        let mut garbagio = "garbagio";
        hello2 = *test(&mut hello, garbagio);
        garbagio = "new garbagio";
        println!("hello2: {:?}", hello2);
    }
    println!("hello2: {hello2}")
}
