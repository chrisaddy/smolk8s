use clap::Command;
use colored::Colorize;

use smolk8s::docker::Docker;
use smolk8s::host::Machine;

fn cli(machine: &Machine) -> Command {
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
    println!("{}", "checking for dependencies...\n".yellow());
    let machine = Machine::new();
    let docker = Docker::new();
    match docker.installed {
        true => {
            let msg = "docker installed:".to_owned();
            println!(
                "{}\n{}",
                msg.green(),
                docker.version.expect("docker version").blue()
            )
        }
        false => println!(
            "{}\n{}\n\t{}",
            "docker not found!!!".red(),
            "Please install Docker first:".blue(),
            "https://docs.docker.com/get-docker/".yellow()
        ),
    }
}

fn main() {
    let machine = Machine::new();
    let matches = cli(&machine).get_matches();
    match matches.subcommand() {
        Some(("minimal", _)) => {
            println!(
                "{} {}[{}]\n",
                "building minimal cluster for".green(),
                machine.os.blue(),
                machine.arch.blue()
            );
            install_minimal();
        }
        _ => unreachable!(),
    }
}
