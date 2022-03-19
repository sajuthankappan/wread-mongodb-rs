use std::error::Error;

use crate::unexpected_error::UnexpectedError;

#[derive(Debug)]
pub enum DataError {
    MongoError(mongodb::error::Error),
    UnexpectedError(UnexpectedError),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataError::MongoError(e) =>  f.write_str(e.to_string().as_str()),
            DataError::UnexpectedError(e) => f.write_str(e.to_string().as_str()),
        }
    }
}

impl Error for DataError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DataError::MongoError(ref e) => Some(e),
            DataError::UnexpectedError(ref e) => Some(e),
        }
    }
}

impl From<mongodb::error::Error> for DataError {
    fn from(e: mongodb::error::Error) -> Self {
        DataError::MongoError(e)
    }
}

impl From<UnexpectedError> for DataError {
    fn from(e: UnexpectedError) -> Self {
        DataError::UnexpectedError(e)
    }
}
