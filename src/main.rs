mod tui;

use std::{fs, io::Write};
use std::process::Command;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = r#"
          This is a cli tool to create/init new new projects.

          Special thanks to:
          ALT-F4-LLC (creator of )https://github.com/ALT-F4-LLC/kickstart.nix ☺
"#)]
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
#[command(author, version, about, long_about = "Create a project using kickstart.nix")]
struct CreateArgs{
    #[arg(short, long,required=false,requires="language")]
    dir: Option<String>,

    #[arg(short, long,required=false,requires="dir")]
    language: Option<String>,
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Initialize a project using kickstart.nix")]
struct InitArgs{
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
                    let _ = tui::create().map_err(|err|{
                        eprintln!("There was this error: {:?}", err);
                    });
                },
            }
        },
        Commands::Init(args) => {
            match &args.language {
                Some(language) => {
                    let _ = Command::new("nix")
                        .arg("flake")
                        .arg("init")
                        .arg("-t")
                        .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}",language))
                        .output();
                },
                None => {
                    let _ = tui::init().map_err(|err| {
                        eprintln!("There was this error: {:?}", err);
                    });
                },
            }
        },
    }
}
