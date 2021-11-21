use std::{thread, time};
use rand::Rng;

fn main() {
    
    // create a vector of threads
    let mut children = vec![];

    // create 12 threads that sleep for a random amount of time between 0 and 60 seconds
    for i in 0..12 {
        // create a new thread
        children.push(thread::spawn(move || {
            // generate a random number between 0 and 60
            let random_number = rand::thread_rng().gen_range(0..60);

            // create duration from random number
            let sleep_time = time::Duration::from_secs(random_number);

            // sleep for the duration
            thread::sleep(sleep_time);

            // print the thread's name and the random number of seconds it slept for
            println!("this is thread number {}. I slept for {} seconds!", i, random_number);
        }));
    }

    // wait for all threads to finish
    for child in children {
        let _ = child.join();
    }

    println!("main thread exiting");

}
