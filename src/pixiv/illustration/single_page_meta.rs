use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SinglePageMeta {
    original_image_url: String,
}
