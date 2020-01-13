use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct AiracError {
    details: String
}

impl AiracError {
    pub fn new(msg: &str) -> AiracError {
        AiracError {details: msg.to_string()}
    }
}

impl fmt::Display for AiracError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",self.details)
    }
}

impl Error for AiracError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for AiracError {
    fn from(err: ParseIntError) -> Self {
        AiracError::new(err.description())
    }
}