mod tui;

use std::{fs, process::Command};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create(CreatArgs)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CreatArgs{
    #[arg(short, long,required=false)]
    dir: Option<String>,

    #[arg(short, long,required=false,requires="dir")]
    language: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create(args) => {
            match &args.dir {
                Some(dir) => {
                    match &args.language {
                        Some(language) => {
                            fs::create_dir(&dir)
                                .expect("couldnt create directory");
                            let _ = std::env::set_current_dir(dir)
                                .expect("Couldn't go into directory");
                            let _ = Command::new("nix")
                                .arg("flake")
                                .arg("init")
                                .arg("-t")
                                .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}",language))
                                .output();
                        },
                        None => {},
                    }
                },
                None => {
                    let _ = tui::run().map_err(|err|{
                        eprintln!("there was this error: {:?}", err);
                    });
                },
            }
        },
    }
}

