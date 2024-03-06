mod replace;
mod tui;

use std::process::Command;
use std::{env, fs};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = r#"
This is a cli tool to create/init new new projects.

Special thanks to:
ALT-F4-LLC creator of https://github.com/ALT-F4-LLC/kickstart.nex"#
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
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Create a project using kickstart.nix"
)]
struct CreateArgs {
    #[arg(short, long, required = false, requires = "language")]
    name: Option<String>,

    #[arg(short, long, required = false, requires = "dir")]
    language: Option<String>,
}
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Initialize a project using kickstart.nix"
)]
struct InitArgs {
    #[arg(short, long, required = false)]
    language: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create(args) => match &args.name {
            Some(dir) => match &args.language {
                Some(language) => {
                    fs::create_dir(&dir).expect("couldnt create directory");
                    let _ = std::env::set_current_dir(dir).expect("Couldn't go into directory");
                    let _ = Command::new("nix")
                        .arg("flake")
                        .arg("init")
                        .arg("-t")
                        .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}", language))
                        .output();

                    let mut flake = fs::read_to_string("./flake.nix").unwrap();
                    flake = flake.replace("example", &dir);
                    let _ = fs::write("./flake.nix", flake).unwrap();

                    print!("{dir}");
                    replace::run(format!("./{}", dir).as_str(), &dir);
                }
                None => {}
            },
            None => {
                let _ = tui::create().map_err(|err| {
                    eprintln!("There was this error: {:?}", err);
                });
            }
        },
        Commands::Init(args) => match &args.language {
            Some(language) => {
                let _ = Command::new("nix")
                    .arg("flake")
                    .arg("init")
                    .arg("-t")
                    .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}", language))
                    .output();
                let current_dir = env::current_dir().unwrap();
                let binding = current_dir.display().to_string();
                let dir = binding.split("/").last().unwrap();

                let mut flake = fs::read_to_string("./flake.nix").unwrap();
                flake = flake.replace("example", &dir);
                let _ = fs::write("./flake.nix", flake).unwrap();

                replace::run("./", &dir);
            }
            None => {
                let _ = tui::init().map_err(|err| {
                    eprintln!("There was this error: {:?}", err);
                });
            }
        },
    }
}
