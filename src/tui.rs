use crate::{replace, templates::get_template_url};
use std::{
    env, fs,
    io::{self, stdin, stdout, BufRead, Result, Write},
    process::{self, Command},
};

pub fn create() -> Result<()> {
    let name = create_text_inputut("Directory: ".to_string()).unwrap();
    if name.is_empty() {
        eprintln!("Please specify the name.");
        process::exit(1);
    }

    let template = create_text_inputut("Language: ".to_string()).unwrap();
    if template.is_empty() {
        eprintln!("Please specify the template.");
        process::exit(1);
    }

    fs::create_dir(name.clone()).expect("couldnt create directory");
    let _ = std::env::set_current_dir(&name).expect("Couldn't go into directory");
    let _ = Command::new("nix")
        .args(["flake","init","-t"])
        .arg(get_template_url(&template).unwrap())
        .output();
    replace::run("./", &name);

    Ok(())
}

pub fn init() -> Result<()> {
    let template = create_text_inputut("Language: ".to_string()).unwrap();
    if template.is_empty() {
        eprintln!("Please specify the language.");
        process::exit(1);
    }

    let _ = Command::new("nix")
        .arg("flake")
        .arg("init")
        .arg("-t")
        .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}", template))
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
