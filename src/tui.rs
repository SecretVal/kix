use std::{io::{stdout, Result,  self, stdin,Write,BufRead}, fs, process::Command};

pub fn create() -> Result<()> {
    let dir = create_text_inputut("Directory: ".to_string()).unwrap();
    let language = create_text_inputut("Language: ".to_string()).unwrap();
    fs::create_dir(dir.clone())
        .expect("couldnt create directory");
    let _ = std::env::set_current_dir(dir)
        .expect("Couldn't go into directory");
    let _ = Command::new("nix")
        .arg("flake")
        .arg("init")
        .arg("-t")
        .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}",language))
        .output();
    return Ok(())
}

pub fn init() -> Result<()> {
    let language = create_text_inputut("Language: ".to_string()).unwrap();
    let _ = Command::new("nix")
        .arg("flake")
        .arg("init")
        .arg("-t")
        .arg(format!("github:ALT-F4-LLC/kickstart.nix#{}",language))
        .output();
    Ok(())
}


fn create_text_inputut(prompt: String) -> Result<String>{
    let mut handler = stdin().lock();
    stdout().write(format!("{}",prompt).as_bytes())?;
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    let _ = handler.read_line(&mut buffer);
    Ok(buffer.trim().to_string())
}
