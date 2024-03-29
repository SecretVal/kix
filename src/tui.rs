use crate::replace;
use std::{
    env, fs,
    io::{self, stdin, stdout, BufRead, Result, Write},
    process::{self, Command},
};

pub fn create() -> Result<()> {
    let dir = create_text_inputut("Directory: ".to_string()).unwrap();
    if dir.is_empty() {
        eprintln!("Please specify the directory.");
        process::exit(1);
    }

    let language = create_text_inputut("Language: ".to_string()).unwrap();
    if language.is_empty() {
        eprintln!("Please specify the language.");
        process::exit(1);
    }

    fs::create_dir(dir.clone()).expect("couldnt create directory");
    let _ = std::env::set_current_dir(&dir).expect("Couldn't go into directory");
    let _ = Command::new("nix")
        .arg("flake")
        .arg("init")
        .arg("-t")
        .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}", language))
        .output();
    replace::run("./", &dir);

    Ok(())
}

pub fn init() -> Result<()> {
    let language = create_text_inputut("Language: ".to_string()).unwrap();
    if language.is_empty() {
        eprintln!("Please specify the language.");
        process::exit(1);
    }

    let _ = Command::new("nix")
        .arg("flake")
        .arg("init")
        .arg("-t")
        .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}", language))
        .output();
    let current_dir = env::current_dir().unwrap();
    let binding = current_dir.display().to_string();
    let dir = binding.split("/").last().unwrap();

    replace::run("./", &dir);

    Ok(())
}

fn create_text_inputut(prompt: String) -> Result<String> {
    let mut handler = stdin().lock();
    stdout().write(format!("{}", prompt).as_bytes())?;
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    let _ = handler.read_line(&mut buffer);
    Ok(buffer.trim().to_string())
}
