use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufReader, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    name: String,
    fields: HashMap<String, String>,
}

impl Profile {
    pub fn new(name: String, fields: HashMap<String, String>) -> Self {
        Self { name, fields }
    }

    pub fn read_profiles<P: AsRef<Path>>(path: P) -> Vec<Self> {
        let file = match File::open(path) {
            Err(_why) => {
                println!("No previous data found, assuming no profiles");
                return Vec::new();
            }
            Ok(file) => file,
        };

        match serde_json::from_reader(BufReader::new(file)) {
            Ok(profiles) => profiles,
            Err(why) => panic!("Failed to deserialize. {}", why),
        }
    }

    pub fn write_profiles<P: AsRef<Path>>(profiles: Vec<Self>, path: P) {
        let serialized_data = serde_json::to_string_pretty(&profiles).unwrap();

        let mut file = match File::create(path) {
            Err(why) => panic!("Unable to create file. {}", why),
            Ok(file) => file,
        };

        file.write_all(serialized_data.as_bytes()).unwrap();
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn fields_stringfied(&self, prefix: &str) -> String {
        let mut s = "".to_string();
        for f in self.fields.iter() {
            s.push_str(format!("{}{}: {}\n", prefix, &f.0, &f.1).as_str());
        }
        s
    }

    pub fn get_by_name(profiles: &[Profile], name: &str) -> Option<usize> {
        profiles.iter().position(|p| p.name().as_str() == name)
    }

    pub fn already_exists(profiles: &[Profile], name: &str) -> bool {
        for p in profiles.iter() {
            if p.name().as_str() == name {
                return true;
            }
        }

        false
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-Profile: {}\n{}",
            self.name,
            self.fields_stringfied("  ")
        )
    }
}
