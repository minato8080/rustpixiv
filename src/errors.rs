use std::error::Error;
use std::fmt;
use std::io::Write;

/// Error returned on failure to authorize with pixiv.
#[derive(Debug)]
pub struct AuthError {
    reason: String,
}

impl AuthError {
    pub fn because<T>(reason: T) -> Self
    where
        T: Into<String>,
    {
        AuthError {
            reason: reason.into(),
        }
    }
}

impl Error for AuthError {
    fn description(&self) -> &str {
        "An error occurred while trying to authenticate."
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occurred while trying to authenticate. Reason: {:?}",
            self.reason
        )
    }
}
