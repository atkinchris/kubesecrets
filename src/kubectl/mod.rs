extern crate serde;
extern crate serde_json;

use self::errors::KubectlError;
use ansi_term::Colour::Blue;
use secrets::Manifest;
use std::error::Error;
use std::str::from_utf8;
use subprocess::{Exec, Redirection};

pub mod errors;

pub fn get_secrets(get_all: bool) -> Result<Manifest, KubectlError> {
  let mut args = vec!["get", "secrets", "-o", "json"];
  if !get_all {
    args.push("-l");
    args.push("managedBy=kubesecrets");
  }

  let output = Exec::cmd("kubectl")
    .args(&args)
    .stdout(Redirection::Pipe)
    .capture()
    .unwrap_or_else(|e| panic!("Failed to execute kubectl: {}", e.description()));

  if !output.exit_status.success() {
    return Err(KubectlError::new("Kubectl failed to retrieve secrets"));
  }

  let manifest: Manifest = serde_json::from_slice(&output.stdout)?;
  return Ok(manifest);
}

pub fn apply(manifest: Manifest, prune: bool) -> Result<(), KubectlError> {
  let json: String = serde_json::to_string_pretty(&manifest)?;
  let mut args = vec!["apply", "-l", "managedBy=kubesecrets"];

  if prune {
    args.push("--prune");
  }

  let output = Exec::cmd("kubectl")
    .args(&args)
    .arg("-f")
    .arg("-")
    .stdin(json.into_bytes())
    .stdout(Redirection::Pipe)
    .capture()
    .unwrap_or_else(|e| panic!("Failed to execute kubectl: {}", e.description()));

  if !output.exit_status.success() {
    return Err(KubectlError::new("Kubectl failed to apply secrets"));
  }

  let stdout = from_utf8(&output.stdout)
    .unwrap_or_else(|e| panic!("Failed to read stdout from kubectl: {}", e.description()));

  print!("{}\n", Blue.paint(stdout));

  return Ok(());
}
