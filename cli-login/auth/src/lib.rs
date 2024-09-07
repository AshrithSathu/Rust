#[derive(PartialEq, Eq, Debug)]
pub enum LoginAction {
    Admin,
    User,
    Denied,
}

pub fn login(username: &str, password: &str) -> LoginAction {
    let username = username.to_lowercase();
    
    if username == "admin" && password == "password" {
        LoginAction::Admin
    } else if username == "user" && password == "password" {
        LoginAction::User
    } else {
        LoginAction::Denied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login() {
        assert_eq!(login("admin", "password"), LoginAction::Admin); // Admin login
        assert_eq!(login("ADMIN", "password"), LoginAction::Admin); // Case-insensitive check for admin
        assert_eq!(login("user", "password"), LoginAction::User);   // User login
        assert_eq!(login("USER", "password"), LoginAction::User);   // Case-insensitive check for user
        assert_eq!(login("admin", "wrong"), LoginAction::Denied);   // Wrong password for admin
        assert_eq!(login("wrong", "password"), LoginAction::Denied); // Invalid username
    }
}