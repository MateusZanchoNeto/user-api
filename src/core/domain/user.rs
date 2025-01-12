#[derive(Debug)]
pub struct User {
    pub name: String,
    pub id: u32,
    pub email: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.email == other.email
    }
}

impl User {
    pub fn new(id: u32, name: String, email: String) -> Self {
        User { id, name, email }
    }

    pub fn validate_email(&self) -> bool {
        self.email.contains('@') && self.email.contains('.')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        assert_eq!(user.validate_email(), true);
    }

    #[test]
    fn test_validate_email_invalid_by_at_sign() {
        let user = User::new(1, "John".to_string(), "john.com".to_string());
        assert_eq!(user.validate_email(), false);
    }

    #[test]
    fn test_validate_email_invalid_by_dot() {
        let user = User::new(1, "John".to_string(), "john@com".to_string());
        assert_eq!(user.validate_email(), false);
    }
}
