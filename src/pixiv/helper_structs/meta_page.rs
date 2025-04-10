use crate::pixiv::helper_structs::image_url::ImageUrl;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetaPage {
    image_urls: ImageUrl,
}

impl MetaPage {
    pub fn image_urls(&self) -> &ImageUrl {
        &self.image_urls
    }
}
