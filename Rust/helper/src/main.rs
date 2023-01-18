mod subcommands;

use anyhow::Result;
use clap::{Arg, ArgAction, Command, ValueHint};

fn app() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .trailing_var_arg(true)
        .subcommand(
            Command::new("new")
                .visible_alias("n")
                .about("Create a new project")
                .arg(
                    Arg::new("path")
                        .value_hint(ValueHint::AnyPath)
                        .help("Directory path of the new project")
                        .required(true),
                )
                .arg(
                    Arg::new("lib")
                        .long("lib")
                        .help("Use a library template")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("bin"),
                )
                .arg(
                    Arg::new("bin")
                        .long("bin")
                        .help("Use a binary (application) template [default]")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("lib"),
                )
                .arg(
                    Arg::new("name")
                        .long("name")
                        .value_name("NAME")
                        .help("Set the resulting package name, defaults to the directory name"),
                ),
        )
        .subcommand(
            Command::new("delete")
                .visible_aliases(["del", "d"])
                .about("Remove a project and its files from workspace")
                .arg(
                    Arg::new("path")
                        .value_hint(ValueHint::AnyPath)
                        .help("Directory path of the project to be removed")
                        .required(true),
                ),
        )
}

fn main() -> Result<()> {
    let m = app().get_matches();

    match m.subcommand() {
        Some(("new", am)) => subcommands::add(am),
        Some(("delete", am)) => subcommands::delete(am),
        _ => Ok(()),
    }
}
