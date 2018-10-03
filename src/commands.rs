extern crate serde_json;

use fs;
use kubectl;
use secrets::{Entry, Item, Manifest};
use std::error::Error;

pub fn pull(get_all: bool, output_file: Option<&str>) -> Result<(), Box<Error>> {
  let manifest = kubectl::get_secrets(get_all)?;
  let entries: Vec<Entry> = manifest.items.into_iter().map(Entry::from_item).collect();
  let json: String = serde_json::to_string_pretty(&entries).unwrap();

  if output_file.is_some() {
    fs::write_file(output_file.unwrap(), &json)?;
  } else {
    println!("{}", json);
  }

  return Ok(());
}

pub fn push(input_file: &str) -> Result<(), Box<Error>> {
  let input = fs::read_file(input_file)?;
  let entries: Vec<Entry> = serde_json::from_str(&input)
    .unwrap_or_else(|e| panic!("couldn't parse input file, {}", e.description()));
  let items: Vec<Item> = entries.into_iter().map(Item::from_entry).collect();
  let manifest = Manifest::from_items(items);

  println!("Please enter 'yes' to continue applying secrets.");
  let line: String = read!("{}\n");

  if line == "yes" {
    kubectl::apply(manifest)?;
  }

  return Ok(());
}
