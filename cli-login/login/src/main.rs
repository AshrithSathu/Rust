use auth::{login, read_line};
use std::io::{self, Write}; // Import io for writing

fn main() {
    println!("Hello, world!");
    println!("Please login\n");

    print!("Username: ");
    io::stdout().flush().unwrap(); // Ensure "Username" is printed before taking input
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = read_line(&username); // Trim the input using the `read_line` function

    print!("Password: ");
    io::stdout().flush().unwrap(); // Ensure "Password" is printed before taking input
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");
    let password = read_line(&password); // Trim the input using the `read_line` function

    if login(&username, &password) {
        println!("Login successful!");
    } else {
        println!("Login failed!");
    }
}