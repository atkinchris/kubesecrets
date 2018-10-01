use fs;
use kubectl;
use std::error::Error;

pub fn pull(output_file: String) -> Result<(), Box<Error>> {
  let response = kubectl::get_secrets()?;
  return fs::write_json(output_file, response);
}
