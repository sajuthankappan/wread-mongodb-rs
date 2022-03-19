use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct UnexpectedError {
    pub error_message: String,
}

impl UnexpectedError {
    pub fn new(error_message: String) -> Self {
        UnexpectedError {
            error_message: error_message,
        }
    }
}

impl fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected error: {}", &self.error_message)
    }
}

impl Error for UnexpectedError {}
