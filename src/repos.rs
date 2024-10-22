#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repo {
    pub(crate) url: String,
    pub(crate) priority: u8,
}

impl Repo {
    pub fn new(url: String, priority: u8) -> Self {
        Self { url, priority }
    }

    pub fn get_template_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_template_url_from_repo(&self, template: String) -> String {
        format!("{}#{}", self.url, template)
    }
}
