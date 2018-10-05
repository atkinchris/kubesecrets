extern crate serde;
extern crate serde_json;

use self::errors::KubectlError;
use ansi_term::Colour::Blue;
use secrets::Manifest;
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub mod errors;

pub fn get_secrets(get_all: bool) -> Result<Manifest, KubectlError> {
  let mut command = Command::new("kubectl");

  command
    .stderr(Stdio::inherit())
    .arg("get")
    .arg("secrets")
    .arg("-o")
    .arg("json");

  if !get_all {
    command.arg("-l").arg("managedBy=kubesecrets");
  }

  let result = command
    .output()
    .unwrap_or_else(|e| panic!("Failed to execute kubectl: {}", e.description()));

  if !result.status.success() {
    return Err(KubectlError::new("Kubectl failed to retrieve secrets"));
  }

  let manifest: Manifest = serde_json::from_slice(&result.stdout)?;
  return Ok(manifest);
}

pub fn apply(manifest: Manifest) -> Result<(), KubectlError> {
  let json: String = serde_json::to_string_pretty(&manifest)?;
  let command = Command::new("kubectl")
    .arg("apply")
    .arg("-f")
    .arg("-")
    .stderr(Stdio::inherit())
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .unwrap_or_else(|e| panic!("Failed to execute kubectl: {}", e.description()));

  command
    .stdin
    .unwrap()
    .write_all(json.as_bytes())
    .unwrap_or_else(|e| panic!("Failed to pipe stdin to kubectl: {}", e.description()));

  let mut result = String::new();
  match command.stdout.unwrap().read_to_string(&mut result) {
    Err(e) => panic!("Failed to read stdout from kubectl: {}", e.description()),
    Ok(_) => print!("{}\n", Blue.paint(result)),
  };

  return Ok(());
}
