use std::fmt;

fn main() {
    let number = rand::thread_rng().gen_range(1..=5);
    println!("The random number is {}", number);
}
