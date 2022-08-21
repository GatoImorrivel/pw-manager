mod profile;

use clap::{arg, Command};

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("create", _)) => {
           println!("Creating new profile!");
        }
        Some(("edit", _)) => {
           println!("Editing profile");
        }
        Some(("delete", _)) => {
           println!("Deleting profile");
        }
        _ => unreachable!(),
    }
}

fn cli() -> Command<'static> {
    Command::new("pw")
        .about("Simple CLI password manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("create")
                .about("Creates a new app profile")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Profile name")),
        )
        .subcommand(
            Command::new("edit")
                .about("Edits a profile")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Profile name")),
        )
        .subcommand(
            Command::new("delete")
                .about("deletes a profile")
                .arg_required_else_help(true)
                .arg(arg!(<NAME> ... "Profile name")),
        )
}
