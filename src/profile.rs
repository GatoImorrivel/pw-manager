use std::{
    collections::HashMap,
    fs::{File},
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
}
