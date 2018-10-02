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
    return fs::write_file(output_file.unwrap(), json);
  }

  println!("{}", json);
  Ok(())
}

pub fn push(input_file: &str) -> Result<(), Box<Error>> {
  let input = fs::read_file(input_file)?;
  let entries: Vec<Entry> = match serde_json::from_str(&input) {
    Err(why) => panic!("couldn't parse input file, {}", why.description()),
    Ok(e) => e,
  };
  let items: Vec<Item> = entries.into_iter().map(Item::from_entry).collect();
  let manifest = Manifest::from_items(items);
  let json: String = serde_json::to_string_pretty(&manifest).unwrap();

  println!("{}", json);
  Ok(())
}
