pub mod data_stores;
pub mod errors;
pub mod user;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, ()> {
        if email.trim().len() == 0 {
            return Err(());
        }
        if !email.contains("@") {
            return Err(());
        }

        Ok(Email(email.to_string()))
    }

    pub fn to_string(self) -> String {
        self.0.to_owned().to_string()
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct Password(String);
impl Password {
    pub fn parse(password: &str) -> Result<Self, ()> {
        if password.trim().len() < 8 {
            return Err(());
        }

        Ok(Password(password.to_string()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_is_blank() {
        assert!(matches!(Email::parse(""), Err(())))
    }

    #[test]
    fn test_email_is_missing_at_symbol() {
        assert!(matches!(Email::parse("aaaa.com"), Err(())))
    }

    #[test]
    fn test_email_parses() {
        let email = "aaa@aa.com";
        assert!(matches!( Email::parse(email), Ok(_)))
    }

    #[test]
    fn test_password_is_blank() {
        assert!(matches!(Password::parse(""), Err(())))
    }

    #[test]
    fn test_password_parses() {
        assert!(matches!( Password::parse("password"), Ok(_)))
    }
}
