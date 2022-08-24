use std::{
    collections::HashMap,
    fs::{File, Permissions},
    io::{BufRead, BufReader, BufWriter, Write},
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

    pub fn read_profiles(file: &File) -> Vec<Self> {
        if BufReader::new(file).lines().count() == 0 {
            println!("No previous data found.");
            return Vec::new();
        }

        match serde_json::from_reader(BufReader::new(file)) {
            Ok(profiles) => profiles,
            Err(why) => panic!("Failed to deserialize. {}", why)
        }
    }

    pub fn write_profiles(profiles: Vec<Self>, file: &mut std::fs::File) {
        let serialized_data = serde_json::to_string_pretty(&profiles).unwrap();

        file.write(serialized_data.as_bytes()).unwrap();
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
