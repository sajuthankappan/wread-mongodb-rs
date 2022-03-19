use axum::http::StatusCode;
use axum_helpers::error_mapper::ErrorMapper;
use mongodb::error::{ErrorKind, WriteFailure};

use super::data_error::DataError;

impl ErrorMapper for DataError {
    fn map_error(&self) -> (axum::http::StatusCode, String) {
        match self {
            DataError::MongoError(err) => match *err.kind.clone() {
                ErrorKind::Write(WriteFailure::WriteError(write_error)) => {
                    if write_error.code == 11000 {
                        (StatusCode::CONFLICT, self.to_string())
                    } else {
                        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
                    }
                }
                _err => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            },
            DataError::UnexpectedError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        }
    }
}
