fn main() {
    // Randomly generated integer between 0 and 9
    let number = rand::thread_rng().gen_range(0, 9);
    
    println!("Random number: {}", number);
}
