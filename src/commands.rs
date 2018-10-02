extern crate serde_json;

use fs;
use kubectl;
use secrets::SecretOutput;
use std::error::Error;

pub fn pull(get_all: bool, output_file: Option<&str>) -> Result<(), Box<Error>> {
  let response = kubectl::get_secrets(get_all)?;

  let entries: Vec<SecretOutput> = response
    .items
    .into_iter()
    .map(SecretOutput::from_entry)
    .collect();
  let json: String = serde_json::to_string_pretty(&entries).unwrap();

  if output_file.is_some() {
    return fs::write_file(output_file.unwrap(), json);
  }

  println!("{}", json);
  Ok(())
}
