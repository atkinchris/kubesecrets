extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct SecretMetaData {
  name: String,
  namespace: String,
  #[serde(default)]
  labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SecretEntry {
  api_version: String,
  kind: String,
  #[serde(rename = "type")]
  entry_type: String,
  data: HashMap<String, String>,
  metadata: SecretMetaData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretResponse {
  api_version: String,
  items: Vec<SecretEntry>,
  kind: String,
}
