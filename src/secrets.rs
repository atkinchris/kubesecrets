extern crate serde;
extern crate serde_json;

use b64::{decode, encode};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
  name: String,
  namespace: String,
  #[serde(default)]
  labels: HashMap<String, String>,
  data: HashMap<String, String>,
}

impl Entry {
  pub fn from_item(item: Item) -> Entry {
    Entry {
      name: item.metadata.name,
      namespace: item.metadata.namespace,
      labels: item.metadata.labels,
      data: decode(item.data),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
  name: String,
  namespace: String,
  #[serde(default)]
  labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
  pub api_version: String,
  pub kind: String,
  #[serde(rename = "type")]
  pub entry_type: String,
  pub data: HashMap<String, String>,
  pub metadata: MetaData,
}

impl Item {
  pub fn from_entry(entry: Entry) -> Item {
    Item {
      api_version: "v1".to_string(),
      kind: "Secret".to_string(),
      entry_type: "Opaque".to_string(),
      data: encode(entry.data),
      metadata: MetaData {
        name: entry.name,
        namespace: entry.namespace,
        labels: entry.labels,
      },
    }
  }
}

impl Eq for Item {}
impl PartialEq for Item {
  fn eq(&self, other: &Item) -> bool {
    self.metadata.name == other.metadata.name
  }
}

impl Hash for Item {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.metadata.name.hash(state);
  }
}

pub trait IntoNames {
  fn into_names(self) -> Vec<String>;
}

impl<'a> IntoNames for Vec<&'a Item> {
  fn into_names(self) -> Vec<String> {
    self.iter().map(|i| i.metadata.name.to_string()).collect()
  }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
  pub api_version: String,
  pub items: Vec<Item>,
  pub kind: String,
}

impl Manifest {
  pub fn from_items(items: Vec<Item>) -> Manifest {
    Manifest {
      api_version: "v1".to_string(),
      kind: "List".to_string(),
      items,
    }
  }
}
