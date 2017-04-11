// That imports crates
extern crate rand;

// And that just brings stuff in namespaces to the main one
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // gen_range (x,y) includes x, excludes y
    let secret = rand::thread_rng()
        .gen_range(1, 101);

    // loop statement begins an infinite loop
    loop {
        println!("Please input your guess.");

        let mut guess : String = String::new();

        // read_line returns an io::Result object that is a failure on
        // empty input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // guess was a string, get a new integer value
        // We also get a Result object that fails on
        // a malformed input. We pattern match on the result
        // instead of using the expect method
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };

        // cmp method on numbers returns an ordering object
        // match statement is a matching operator
        // match statement ends with commas, mimicking an enum definition
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => { 
                println!("Size does matter after all");
                break;
            }
        }
    }
}
