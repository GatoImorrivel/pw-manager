use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    name: String,
    fields: HashMap<String, String>
}

impl Profile {
    pub fn new(name: String, fields: HashMap<String, String>) -> Self {
        Self { name, fields }
    }
}
