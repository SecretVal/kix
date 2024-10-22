#![allow(dead_code)]
mod config;
mod replace;
mod repos;
mod templates;
mod tui;

use std::process::{exit, Command};
use std::{env, fs};

use clap::{Parser, Subcommand};
use config::*;
use piglog::*;
use templates::get_template_url;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"
This is a cli tool to create/init new new projects.

Special thanks to:
ALT-F4-LLC creator of https://github.com/ALT-F4-LLC/kickstart.nix"#
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create(CreateArgs),
    Init(InitArgs),
    Update,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Create a project using kickstart.nix"
)]
struct CreateArgs {
    #[arg(short, long, required = false, requires = "template")]
    name: Option<String>,

    #[arg(short, long, required = false, requires = "name")]
    template: Option<String>,
}
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Initialize a project using any nixo"
)]
struct InitArgs {
    #[arg(short, long, required = false)]
    template: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create(args) => match &args.name {
            Some(name) => match &args.template {
                Some(template_name) => {
                    let template = get_template_url(template_name);
                    if template.is_none() {
                        error!("Not a valid template");
                        exit(1);
                    }
                    fs::create_dir(&name).expect("couldnt create directory");
                    let _ = std::env::set_current_dir(name).expect("Couldn't go into directory");
                    let _ = Command::new("nix")
                        .args(["flake", "init", "-t"])
                        .arg(template.unwrap())
                        .output();
                    replace::run("./", &name);
                }
                None => {}
            },
            None => {
                let _ = tui::create().map_err(|err| {
                    eprintln!("There was this error: {:?}", err);
                });
            }
        },
        Commands::Init(args) => match &args.template {
            Some(template_name) => {
                let template = get_template_url(template_name);
                if template.is_none() {
                    error!("Not a valid template");
                    exit(1);
                }
                let _ = Command::new("nix")
                    .args(["flake", "init", "-t"])
                    .arg(template.unwrap())
                    .output();
                let current_dir = env::current_dir().unwrap();
                let binding = current_dir.display().to_string();
                let dir = binding.split("/").last().unwrap();

                replace::run("./", &dir);
            }
            None => {
                let _ = tui::init().map_err(|err| {
                    eprintln!("There was this error: {:?}", err);
                });
            }
        },
        Commands::Update => {
            let config: Config =
                serde_json::from_str(read_config().as_str()).expect("Could not parse Json");
            info!("Updating repos:");
            for repo in config.repos {
                info!("Updating: {}", repo.url);
                let _ = Command::new("nix")
                    .args(["flake", "update", "--flake", repo.url.as_str()])
                    .output();
            }
        }
    }
}
