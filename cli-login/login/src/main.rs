use auth::login;
use std::io::{self, Write}; // Import io for writing

fn main() {
    let mut tries = 0;
    let max_tries = 3;

    while tries < max_tries {
        println!("Please login\n");

        // Prompt for Username
        print!("Username: ");
        io::stdout().flush().unwrap(); // Ensure "Username" is printed before taking input
        let mut username = String::new();
        io::stdin().read_line(&mut username).expect("Failed to read username");
        let username = username.trim(); // Trim the input to remove newlines and spaces

        // Prompt for Password
        print!("Password: ");
        io::stdout().flush().unwrap(); // Ensure "Password" is printed before taking input
        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to read password");
        let password = password.trim(); // Trim the input to remove newlines and spaces

        // Call the login function
        match login(username, password) {
            auth::LoginAction::Admin => {
                println!("Welcome Admin!");
                break;
            }
            auth::LoginAction::User => {
                println!("Welcome User!");
                break;
            }
            auth::LoginAction::Denied => {
                println!("Invalid username or password. Please try again.");
                tries += 1;
            }
        }
    }
}