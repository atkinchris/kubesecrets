extern crate serde_json;

use ansi_term::Colour::{Green, Red, White};
use difference::difference;
use duplicates::find_duplicates;
use errors::ApplicationError;
use fs;
use kubectl;
use secrets::{Entry, IntoNames, Item, Manifest};
use std::error::Error;

pub fn pull(get_all: bool, output_file: Option<&str>) -> Result<(), ApplicationError> {
    let manifest = kubectl::get_secrets(get_all)?;
    let entries: Vec<Entry> = manifest.items.into_iter().map(Entry::from_item).collect();
    let json: String = serde_json::to_string_pretty(&entries).unwrap();

    if output_file.is_some() {
        fs::write_file(output_file.unwrap(), &json)?;
        let output = format!(
            "Wrote {} secrets to \"{}\".",
            entries.len(),
            output_file.unwrap()
        );
        println!("{}", Green.paint(output));
    } else {
        println!("{}", json);
    }

    return Ok(());
}

pub fn push(input_file: &str, prune: bool) -> Result<(), ApplicationError> {
    let input = fs::read_file(input_file)?;
    let entries: Vec<Entry> = serde_json::from_str(&input)
        .unwrap_or_else(|e| panic!("couldn't parse input file, {}", e.description()));

    {
        let duplicates = find_duplicates(&entries);
        if duplicates.is_some() {
            let names: Vec<String> = duplicates
                .unwrap()
                .into_iter()
                .map(|i| i.name.to_owned())
                .collect();
            let message = format!("Duplicate secrets found:\n{}", names.join("\n"));
            return Err(ApplicationError::new(&message));
        }
    }

    let items: Vec<Item> = entries.into_iter().map(Item::from_entry).collect();
    let items_length = items.len();

    {
        let existing = kubectl::get_secrets(false)?.items;
        let (unchanged, added, deleted) = difference(&existing, &items);

        println!("Read {} secrets from \"{}\".\n", items_length, input_file);

        if unchanged.len() > 0 {
            println!(
                "To be updated:\n{}\n",
                White.bold().paint(unchanged.into_names().join("\n"))
            );
        }

        if added.len() > 0 {
            println!(
                "To be added:\n{}\n",
                Green.bold().paint(added.into_names().join("\n"))
            );
        }

        if deleted.len() > 0 && prune {
            println!(
                "To be pruned:\n{}\n",
                Red.bold().paint(deleted.into_names().join("\n"))
            );
        }
    }

    println!(
        "Please type '{}' to continue applying secrets.",
        White.bold().paint("yes")
    );
    let line: String = read!("{}\n");

    if line == "yes" {
        println!();
        let manifest = Manifest::from_items(items);
        kubectl::apply(manifest, prune)?;
        let message = format!("Applied {} secrets to Kubernetes.", items_length);
        println!("{}", Green.paint(message));
    }

    return Ok(());
}
