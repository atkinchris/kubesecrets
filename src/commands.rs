extern crate serde_json;

use fs;
use kubectl;
use secrets::{SecretEntry, SecretOutput, SecretResponse};
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

pub fn push(input_file: &str) -> Result<(), Box<Error>> {
  let input = fs::read_file(input_file)?;
  let entries: Vec<SecretOutput> = match serde_json::from_str(&input) {
    Err(why) => panic!("couldn't parse input file, {}", why.description()),
    Ok(e) => e,
  };
  let items: Vec<SecretEntry> = entries.into_iter().map(SecretEntry::from_output).collect();
  let manifest = SecretResponse::from_items(items);
  let json: String = serde_json::to_string_pretty(&manifest).unwrap();

  println!("{}", json);
  Ok(())
}
