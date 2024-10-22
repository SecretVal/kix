use crate::{
    replace,
    templates::{get_template_url, get_templates},
};
use fzf_wrapped::{run_with_output, Fzf};
use piglog::*;
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

    let mut templates: Vec<String> = vec![];
    for template in get_templates() {
        templates.push(template.1);
    }
    let template = run_with_output(Fzf::default(), templates).expect("Could not get user input");
    if template.is_empty() {
        error!("Not a valid template");
        process::exit(1);
    }

    fs::create_dir(name.clone()).expect("couldnt create directory");
    let _ = std::env::set_current_dir(&name).expect("Couldn't go into directory");
    let _ = Command::new("nix")
        .args(["flake", "init", "-t"])
        .arg(get_template_url(&template).unwrap())
        .output();
    replace::run("./", &name);

    Ok(())
}

pub fn init() -> Result<()> {
    let mut templates: Vec<String> = vec![];
    for template in get_templates() {
        templates.push(template.1);
    }
    let template = run_with_output(Fzf::default(), templates).expect("Could not get user input");
    if template.is_empty() {
        error!("Not a valid template");
        process::exit(1);
    }

    let _ = Command::new("nix")
        .args(["flake", "init", "-t"])
        .arg(get_template_url(&template).unwrap())
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
