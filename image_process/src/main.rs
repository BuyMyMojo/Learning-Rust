use std::time::{Instant};
use rayon::prelude::*;

fn main() {
    // let Ree = [[1, 1 ,1], [5, 5, 5], [8, 5, 2]];
    let now = Instant::now();
    let mut list: Vec<u32> = (1..=1000000).collect();

    list.par_iter_mut().for_each(|n| 
    if *n % 15 == 0 {
        println!("fizzbuzz");
    } else if *n % 3 == 0 {
        println!("fizz");
    } else if *n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", *n);
    }
    );

    // for n in 1..=1000000 {
    //     if n % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if n % 3 == 0 {
    //         println!("fizz");
    //     } else if n % 5 == 0 {
    //         println!("buzz");
    //     } else {
    //         println!("{}", n);
    //     }
    // }
    println!("{}", now.elapsed().as_secs());

}