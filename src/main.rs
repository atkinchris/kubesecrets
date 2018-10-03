#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate base64;

mod b64;
mod commands;
mod fs;
mod kubectl;
mod secrets;

use clap::{AppSettings, Arg, SubCommand};

fn app() -> Result<(), Box<std::error::Error>> {
    let matches = app_from_crate!()
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("pull")
                .about("Pull secrets from kubernetes")
                .arg(Arg::from_usage("-o, --output [FILE] 'output to file'"))
                .arg(Arg::from_usage("-a, --all 'get all secrets'")),
        ).subcommand(
            SubCommand::with_name("push")
                .about("Push secrets to kubernetes")
                .arg(
                    Arg::with_name("input")
                        .help("input file containing secrets")
                        .required(true),
                ).arg(Arg::from_usage(
                    "-d, --delete 'remove secrets not in input'",
                )),
        ).get_matches();

    match matches.subcommand() {
        ("pull", Some(pull_matches)) => {
            let output = pull_matches.value_of("output");
            let get_all = pull_matches.is_present("all");
            return commands::pull(get_all, output);
        }
        ("push", Some(push_matches)) => {
            let input = push_matches.value_of("input").unwrap();
            return commands::push(input);
        }
        _ => unreachable!(),
    }
}

fn main() {
    if let Err(err) = app() {
        eprintln!("\n{}", &err.description());
        std::process::exit(1);
    }
}
