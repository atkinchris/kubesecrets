extern crate serde;
extern crate serde_json;

use self::errors::KubectlError;
use secrets::SecretResponse;
use std::process::Command;

pub mod errors;

pub fn get_secrets(get_all: bool) -> Result<SecretResponse, KubectlError> {
  let mut command = Command::new("kubectl");

  command.arg("get").arg("secrets").arg("-o").arg("json");

  if !get_all {
    command.arg("-l").arg("managedBy=kubesecrets");
  }

  let result = command
    .output()
    .unwrap_or_else(|e| panic!("Failed to execute process: {}", e));

  if !result.status.success() {
    let error = String::from_utf8_lossy(&result.stderr);
    return Err(KubectlError::new(&error));
  }

  let response: SecretResponse = serde_json::from_slice(&result.stdout)?;
  return Ok(response);
}
