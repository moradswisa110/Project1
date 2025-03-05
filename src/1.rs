
// Create a new struct to store user data
struct User {
    id: i32,
    name: String,
}

// Define a function to get a random user from a list
fn get_random_user(users: Vec<User>) -> User {
    // Use the `rand` crate to generate a random index into the vector of users
    let rand_index = rand::thread_rng().gen_range(0..users.len());

    // Return the user at the randomly generated index
    users[rand_index]
}

// Test the function with some sample data
let users = vec![
    User { id: 1, name: "Alice".to_string() },
    User { id: 2, name: "Bob".to_string() },
    User { id: 3, name: "Charlie".to_string() },
];
let random_user = get_random_user(users);
println!("{}", random_user.name);