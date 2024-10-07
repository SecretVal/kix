#![allow(dead_code)]

use std::process::Command;

use crate::config::{read_config, Config};
use crate::repos::Repo;


pub fn get_templates() -> Vec<(Repo, String, u8)>{
    let config: Config =
    serde_json::from_str(read_config().as_str()).expect("Could not parse Json");

    let mut templates: Vec<(Repo, String, u8)> = vec![];
    for template in config.repos {
        let json: serde_json::Value = serde_json::from_str(String::from_utf8(
            Command::new("nix")
                .args(["flake", "show", &template.get_template_url(), "--json"])
                .output()
                .unwrap()
            .stdout,
        )
            .unwrap().as_str()).unwrap();
        for (key, _) in json["templates"].as_object().unwrap() {
            templates.push((template.clone(), key.to_string(), template.priority));
        }
    }
    templates.sort_by(|a,b| b.2.cmp(&a.2));
    templates
}

pub fn get_template_url(name: &str) -> Option<String>{
    for (t, k, _) in get_templates(){
        if k == name.to_string() {
            return Some(format!("{}#{}", t.url, k));
        }
    }
    None
}


