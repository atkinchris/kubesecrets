use base64::decode;
use std::collections::HashMap;
use std::str::from_utf8;

pub fn b64_decode(input: HashMap<String, String>) -> HashMap<String, String> {
  let mut output = HashMap::new();

  for (key, val) in input.iter() {
    let value = decode(val).unwrap();
    output.insert(key.to_string(), from_utf8(&value).unwrap().to_string());
  }

  return output;
}
