use std::{fs, path::PathBuf, process};

pub fn run(dir: &str, r: &str) {
    let files = read_dir(dir);
    process_files(files, r);
}

fn read_dir(path: &str) -> Vec<PathBuf> {
    let dir = fs::read_dir(path);
    let mut files: Vec<PathBuf> = Vec::new();
    for file in dir.unwrap() {
        if file.is_ok() {
            if file
                .as_ref()
                .unwrap()
                .path()
                .strip_prefix("./")
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(".")
            {
                continue;
            }
            // TODO: maybe implement stuff like target and build
            if file
                .as_ref()
                .unwrap()
                .path()
                .strip_prefix("./")
                .unwrap()
                .starts_with("target")
            {
                continue;
            }
            if file.as_ref().unwrap().path().is_file() {
                files.push(file.as_ref().unwrap().path());
            } else if file.as_ref().unwrap().path().is_dir() {
                files.append(&mut read_dir(file.unwrap().path().to_str().unwrap()));
            }
        } else {
            eprintln!("Couldn't read a file");
            process::exit(1);
        }
    }
    files
}

fn process_files(files: Vec<PathBuf>, r: &str) {
    for file in files {
        let content = fs::read_to_string(file.clone());
        if !content.is_ok() {
            eprintln!("Couldn't read a file");
            process::exit(1);
        }
        let _ = fs::write(file.clone(), content.unwrap().replace("example", r));
    }
}
