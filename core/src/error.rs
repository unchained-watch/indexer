use hex::FromHexError;
use std::convert::From;
use std::error::Error;
use std::fmt::{self, Display};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ServiceError {
    message: String,
    cause: Option<Box<dyn Error>>,
}

impl ServiceError {
    fn new(message: String, cause: Option<Box<dyn Error>>) -> ServiceError {
        ServiceError { message, cause }
    }
}

impl Error for ServiceError {
    fn description(&self) -> &str {
        &self.message
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|c| c.as_ref())
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref cause) = self.cause {
            write!(f, ": {}", cause)?;
        }
        Ok(())
    }
}

impl From<&str> for ServiceError {
    fn from(message: &str) -> ServiceError {
        ServiceError::new(message.to_string(), None)
    }
}

impl From<ParseIntError> for ServiceError {
    fn from(error: ParseIntError) -> ServiceError {
        ServiceError::new(error.to_string(), Some(Box::new(error)))
    }
}

impl From<FromHexError> for ServiceError {
    fn from(error: FromHexError) -> ServiceError {
        ServiceError::new(error.to_string(), Some(Box::new(error)))
    }
}

impl From<rustc_hex::FromHexError> for ServiceError {
    fn from(error: rustc_hex::FromHexError) -> ServiceError {
        ServiceError::new(error.to_string(), Some(Box::new(error)))
    }
}

impl From<mongodb::error::Error> for ServiceError {
    fn from(error: mongodb::error::Error) -> ServiceError {
        ServiceError::new(error.to_string(), Some(Box::new(error)))
    }
}

impl From<web3::Error> for ServiceError {
    fn from(error: web3::Error) -> ServiceError {
        ServiceError::new(error.to_string(), Some(Box::new(error)))
    }
}
