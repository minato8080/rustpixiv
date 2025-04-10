use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SingleMetaPage {
    original_image_url: Option<String>,
}

impl SingleMetaPage {
    pub fn original_image_url(&self) -> Option<&String> {
        self.original_image_url.as_ref()
    }
}
