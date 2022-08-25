mod profile;
mod utils;

use clap::{arg, Arg, Command};
use std::{collections::HashMap, io::Write, path::Path};

use profile::Profile;
use utils::{prompt_field, read_line_sanitized};

use crate::utils::contains_profile_by_name;

fn main() {
    let data_path = Path::new("data.json");

    let mut profiles = Profile::read_profiles(&data_path);
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap().to_owned();
            let mut fields: HashMap<String, String> = HashMap::new();

            if contains_profile_by_name(&profiles, name.as_str()) {
                println!("A profile with this name already exists.");
                return;
            }

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
            Profile::write_profiles(profiles, data_path);
        }
        Some(("edit", _)) => {
            println!("Editing profile");
        }
        Some(("delete", sub_matches)) => {
            println!("Deleting profile");
            let name = sub_matches.get_one::<String>("name").unwrap().to_owned();

            let i = match Profile::get_by_name(&profiles, name.as_str()) {
                None => {
                    println!("Can't find profile for deletion");
                    return;
                }
                Some(p) => p,
            };

            profiles.remove(i);
            Profile::write_profiles(profiles, data_path);
        }
        Some(("get", sub_matches)) => {
            println!("Getting profile");

            let name = sub_matches.get_one::<String>("name").unwrap();
            let i = match Profile::get_by_name(&profiles, name.as_str()) {
                None => {
                    println!("Can't get desired profile, not found");
                    return;
                }
                Some(p) => p,
            };

            println!("{}", profiles[i]);
        }
        Some(("list", _)) => {
            for p in profiles.iter() {
                println!("{}", p);
            }
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
                .arg(Arg::new("name").id("name").action(clap::ArgAction::Set)),
        )
        .subcommand(
            Command::new("get")
                .about("gets information from the specified profile")
                .arg_required_else_help(true)
                .arg(Arg::new("name").id("name").action(clap::ArgAction::Set)),
        )
        .subcommand(Command::new("list").about("shows all profiles for a user"))
}
