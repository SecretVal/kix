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
    Create {
        dir: String,
        name: String,
    },
    Version 
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create { dir, name } => {
            let _ = fs::create_dir(dir);
            let _ = std::env::set_current_dir(dir)
                .expect("Couldn't go into directory");
            let _ = Command::new("nix")
                .arg("flake")
                .arg("init")
                .arg("-t")
                .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}",name))
                .output();
        }
        Commands::Version => {
            println!(env!("CARGO_PKG_VERSION"));
        }
    }
}

