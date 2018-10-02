use b64::b64_decode;
use fs;
use kubectl;
use secrets::SecretEntry;
use std::error::Error;

pub fn pull(output_file: String) -> Result<(), Box<Error>> {
  let response = kubectl::get_secrets()?;

  let decoded: Vec<SecretEntry> = response
    .items
    .into_iter()
    .map(|entry| SecretEntry {
      data: b64_decode(entry.data),
      ..entry
    }).collect();

  return fs::write_json(output_file, decoded);
}
