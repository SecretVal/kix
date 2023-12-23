use crossterm::{
    event::{self, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::CrosstermBackend,
    widgets::Paragraph, Terminal 
};
use std::{io::{stdout, Result, Stdout}, fs, process::Command};
pub fn run() -> Result<()> {
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let dir = create_text_inputut(terminal, "Directory: ".to_owned()).unwrap();
    terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let language = create_text_inputut(terminal, "Language: ".to_owned()).unwrap();
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
    Ok(())
}

fn create_text_inputut(mut terminal: Terminal<CrosstermBackend<Stdout>>, prompt: String) -> Result<String>{
    let mut out = String::new(); 
    stdout().execute(EnterAlternateScreen)?; 
    enable_raw_mode()?;
    let _ = terminal.show_cursor();
    loop {
        let _ = terminal.show_cursor();
        let _completed_frame = terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new(prompt.clone() + &out),
                area,
                );
        })?;
        let _ = terminal.show_cursor();
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press{
                    if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c'){
                        stdout().execute(LeaveAlternateScreen)?;
                        disable_raw_mode()?;
                        std::process::exit(0)
                    }
                    match key.code{
                        KeyCode::Enter=> break,
                        KeyCode::Char(' ') => {
                            continue;
                        },
                        KeyCode::Char(a) => {
                            out = out + &String::from(a);
                        },
                        KeyCode::Backspace => {
                            out.pop();
                        },
                        _ => {}, 
                    }
                }
            }
        }
        let _ = terminal.show_cursor();
    }
    let _ = terminal.show_cursor();
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(out)
}
