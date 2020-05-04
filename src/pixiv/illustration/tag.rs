use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Tag {
    name: String,
    translated_name: Option<Vec<String>>,
}
