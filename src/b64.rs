use base64;
use std::collections::HashMap;
use std::str::from_utf8;

pub fn decode(input: HashMap<String, String>) -> HashMap<String, String> {
  let mut output = HashMap::new();

  for (key, val) in input.iter() {
    let value = base64::decode(val).expect(&format!("Error decoding Base64 in {}", key));
    let value_string = from_utf8(&value).expect(&format!("Error representing {} as UTF-8", key));
    output.insert(key.to_string(), value_string.to_string());
  }

  return output;
}

pub fn encode(input: HashMap<String, String>) -> HashMap<String, String> {
  let mut output = HashMap::new();

  for (key, val) in input.iter() {
    let value = base64::encode(val);
    output.insert(key.to_string(), value);
  }

  return output;
}
