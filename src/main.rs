extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod secrets;

use secrets::SecretResponse;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

fn main() {
    let output = Command::new("kubectl")
        .arg("get")
        .arg("secrets")
        .arg("-o")
        .arg("json")
        .output()
        .expect("failed to execute process");

    let response: SecretResponse = serde_json::from_slice(&output.stdout).unwrap();
    let output_json: String = serde_json::to_string_pretty(&response).unwrap();

    let path = Path::new("output.json");
    let display = path.display();
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(output_json.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
