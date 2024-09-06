// Original login function
pub fn login(username: &str, password: &str) -> bool {
    let username = username.to_string().to_lowercase();
    let password = password.to_string().to_lowercase();
    username == "admin" && password == "password"
}

// Refactor read_line to accept an argument for testing purposes
pub fn read_line(input: &str) -> String {
    input.trim().to_string() // Trim the input and return the result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), true);
        assert_eq!(login("admin", "wrong"), false);
        assert_eq!(login("wrong", "password"), false);
    }

    #[test]
    fn test_read_line() {
        let input = "  hello  "; // Input with extra spaces
        assert_eq!(read_line(input), "hello"); // Test the trimmed output
    }
}