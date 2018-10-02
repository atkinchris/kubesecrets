extern crate serde_json;

use b64::b64_decode;
use fs;
use kubectl;
use secrets::SecretEntry;
use std::error::Error;

pub fn pull(get_all: bool, output_file: Option<&str>) -> Result<(), Box<Error>> {
  let response = kubectl::get_secrets(get_all)?;

  let entries: Vec<SecretEntry> = response
    .items
    .into_iter()
    .map(|entry| SecretEntry {
      data: b64_decode(entry.data),
      ..entry
    }).collect();
  let json: String = serde_json::to_string_pretty(&entries).unwrap();

  if output_file.is_some() {
    return fs::write_file(output_file.unwrap(), json);
  }

  println!("{}", json);
  Ok(())
}
