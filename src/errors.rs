extern crate serde_json;

use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub struct ApplicationError {
  details: String,
}

impl ApplicationError {
  pub fn new(msg: &str) -> ApplicationError {
    ApplicationError {
      details: msg.to_string(),
    }
  }

  pub fn description(&self) -> &str {
    &self.details
  }
}

impl fmt::Display for ApplicationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:#?}", self.details)
  }
}

impl Error for ApplicationError {
  fn description(&self) -> &str {
    &self.details
  }
}

impl From<serde_json::Error> for ApplicationError {
  fn from(err: serde_json::Error) -> Self {
    ApplicationError::new(err.description())
  }
}

impl From<io::Error> for ApplicationError {
  fn from(err: io::Error) -> Self {
    ApplicationError::new(err.description())
  }
}
