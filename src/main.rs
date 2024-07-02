use clap::{Arg, Command};
use std::env::consts;

use smolk8s::machine::Machine;

fn cli(machine: Machine) -> Command {
    Command::new("smolk8s")
        .about(format!("let's get you kubing, {}", machine))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("minimal")
                .about("minimal k8s setup, useful for building your own thing from scratch"),
        )
}

fn install_minimal() {
    let machine = Machine::new();
    println!("{:?}", machine);
}

fn main() {
    let machine = Machine::new();
    let matches = cli(machine).get_matches();
    match matches.subcommand() {
        Some(("minimal", _)) => {
            println!("building minimal cluster");
            install_minimal();
        }
        _ => unreachable!(),
    }
}
