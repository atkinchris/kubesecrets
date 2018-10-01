extern crate serde;
extern crate serde_json;

use self::errors::KubectlError;
use secrets::SecretResponse;
use std::process::Command;

pub mod errors;

pub fn get_secrets() -> Result<SecretResponse, KubectlError> {
  let result = Command::new("kubectl")
    .arg("get")
    .arg("secrets")
    .arg("-o")
    .arg("json")
    .output()
    .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

  if !result.status.success() {
    let error = String::from_utf8_lossy(&result.stderr);
    return Err(KubectlError::new(&error));
  }

  let response: SecretResponse = serde_json::from_slice(&result.stdout)?;
  return Ok(response);
}
