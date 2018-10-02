extern crate serde;
extern crate serde_json;

use b64::{decode, encode};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretOutput {
  name: String,
  namespace: String,
  #[serde(default)]
  labels: HashMap<String, String>,
  data: HashMap<String, String>,
}

impl SecretOutput {
  pub fn from_entry(entry: SecretEntry) -> SecretOutput {
    SecretOutput {
      name: entry.metadata.name,
      namespace: entry.metadata.namespace,
      labels: entry.metadata.labels,
      data: decode(entry.data),
    }
  }
}

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

impl SecretEntry {
  pub fn from_output(output: SecretOutput) -> SecretEntry {
    let metadata = SecretMetaData {
      name: output.name,
      namespace: output.namespace,
      labels: output.labels,
    };

    return SecretEntry {
      api_version: "v1".to_string(),
      kind: "Secret".to_string(),
      entry_type: "Opaque".to_string(),
      data: encode(output.data),
      metadata,
    };
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecretResponse {
  pub api_version: String,
  pub items: Vec<SecretEntry>,
  pub kind: String,
}

impl SecretResponse {
  pub fn from_items(items: Vec<SecretEntry>) -> SecretResponse {
    SecretResponse {
      api_version: "v1".to_string(),
      kind: "List".to_string(),
      items,
    }
  }
}
