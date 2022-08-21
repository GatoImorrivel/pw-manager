mod profile;
mod utils;

use std::{collections::HashMap, io::Write};
use clap::{arg, Arg, Command};

use profile::Profile;
use utils::{prompt_field, read_line_sanitized};

fn main() {
    let mut profiles: Vec<Profile> = vec![];
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap().to_owned();
            let mut fields: HashMap<String, String> = HashMap::new();

            println!("Creating new profile!");
            let fields = 'outer: loop {
                let (field_name, field_value) = prompt_field();
                fields.insert(field_name, field_value);

                'inner: loop {
                    print!("Add another field? (y/n): ");
                    std::io::stdout().flush().unwrap();
                    let sanitized_input = read_line_sanitized().to_lowercase();

                    match sanitized_input.as_str() {
                        "y" | "yes" => continue 'outer,
                        "n" | "no" => break 'outer fields,
                        _ => {
                            println!("try again");
                            continue 'inner;
                        }
                    }
                }
            };

            profiles.push(Profile::new(name, fields));
        }
        Some(("edit", _)) => {
            println!("Editing profile");
        }
        Some(("delete", _)) => {
            println!("Deleting profile");
        }
        Some(("get", _)) => {
            println!("Getting profile");
        }
        Some(("list", _)) => {
            println!("{:?}", profiles);
        }
        _ => unreachable!(),
    }
}

fn cli() -> Command<'static> {
    Command::new("pwman")
        .about("Simple CLI password manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("create")
                .about("creates a new app profile")
                .arg_required_else_help(true)
                .arg(Arg::new("name").id("name").action(clap::ArgAction::Set)),
        )
        .subcommand(
            Command::new("edit")
                .about("edits a profile")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Profile name")),
        )
        .subcommand(
            Command::new("delete")
                .about("deletes a profile")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Profile name")),
        )
        .subcommand(
            Command::new("get")
                .about("gets information from the specified profile")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Profile name")),
        )
        .subcommand(Command::new("list").about("shows all profiles for a user"))
}
