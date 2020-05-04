use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SingleMetaPage {
    original_image_url: Option<String>,
}
