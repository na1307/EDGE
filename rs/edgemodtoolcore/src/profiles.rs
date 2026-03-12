use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles {
    default: String,
    profiles: HashMap<String, PathBuf>,
}

impl Profiles {
    pub fn from_json(json_file: impl Read) -> serde_json::Result<Self> {
        serde_json::from_reader(json_file)
    }

    pub fn save_json(&self, json_file: impl Write) -> serde_json::Result<()> {
        serde_json::to_writer_pretty(json_file, self)
    }

    pub fn add_or_modify_profile(&mut self, name: &str, path: PathBuf) {
        self.profiles.insert(name.to_string(), path);
    }

    pub fn remove_profile(&mut self, name: &str) -> Result<(), String> {
        if !self.profiles.contains_key(name) {
            return Err("The specified profile does not exist.".to_string());
        }

        self.profiles.remove(name);

        Ok(())
    }

    pub fn get_default(&self) -> &str {
        &self.default
    }

    pub fn set_default(&mut self, name: &str) -> Result<(), String> {
        if !name.is_empty() && !self.profiles.contains_key(name) {
            return Err("The specified profile does not exist.".to_string());
        }

        self.default = name.to_string();

        Ok(())
    }

    pub fn get_count(&self) -> usize {
        self.profiles.len()
    }

    pub fn get_path_for_profile(&self, name: &str) -> Result<PathBuf, String> {
        self.profiles
            .get(name)
            .cloned()
            .ok_or("The specified profile does not exist.".to_string())
    }
}

impl Display for Profiles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?)
    }
}
