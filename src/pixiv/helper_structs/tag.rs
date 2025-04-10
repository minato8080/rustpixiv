use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tag {
    name: String,
    translated_name: Option<Vec<String>>,
}

impl Tag {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn translated_name(&self) -> &Option<Vec<String>> {
        &self.translated_name
    }
}
