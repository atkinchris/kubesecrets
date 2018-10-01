extern crate serde_json;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct KubectlError {
  details: String,
}

impl KubectlError {
  pub fn new(msg: &str) -> KubectlError {
    KubectlError {
      details: msg.to_string(),
    }
  }
}

impl fmt::Display for KubectlError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:#?}", self.details)
  }
}

impl Error for KubectlError {
  fn description(&self) -> &str {
    &self.details
  }
}

impl From<serde_json::Error> for KubectlError {
  fn from(err: serde_json::Error) -> Self {
    KubectlError::new(err.description())
  }
}
