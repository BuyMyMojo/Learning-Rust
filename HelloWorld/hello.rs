use arguments;

fn hello(foo: bool) {
    if foo == true {
    println!("Hello, world!");
    }
    else {
        println!("Goodbye, world!");
    }
}

fn main() {
    let arguments = std::env::args(); // --no-boo
    let arguments = arguments::parse(arguments).unwrap();

    let boo = arguments.get::<bool>("boo").unwrap();
    hello(boo);
}
