extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretMetaData {
  name: String,
  namespace: String,
  #[serde(default)]
  labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretEntry {
  pub api_version: String,
  pub kind: String,
  #[serde(rename = "type")]
  pub entry_type: String,
  pub data: HashMap<String, String>,
  pub metadata: SecretMetaData,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretResponse {
  pub api_version: String,
  pub items: Vec<SecretEntry>,
  pub kind: String,
}
