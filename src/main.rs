extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod commands;
mod kubectl;
mod secrets;

use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    return commands::pull();
}
