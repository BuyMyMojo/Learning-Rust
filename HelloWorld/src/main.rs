extern crate clap;
use clap::{Arg, App, SubCommand};


fn hello(foo: bool) {
    if foo == true {
    println!("Hello, world!");
    }
    else {
        println!("Goodbye, world!");
    }
}

fn main() {
    let matches = App::new("Hello World With Args")
                          .version("1.0")
                          .author("BuyMyMojo <hello@owenscode.org>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("goodbye")
                               .short("gb")
                               .long("goodbye")
                               .help("set goodbye")
                               .required(false)
                                .takes_value(false))
                          .get_matches();

    
    let boo: bool = !matches.is_present("goodbye");
    hello(boo);
}
