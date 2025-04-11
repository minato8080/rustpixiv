use crate::pixiv::helper_structs::image_url::ImageUrl;

use serde::{Deserialize, Serialize};

/// The user who worked on the illustration (the artist).
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    account: String,
    id: u32,
    is_followed: Option<bool>,
    name: String,
    profile_image_urls: ImageUrl,
}

impl User {
    pub fn account(&self) -> &String {
        &self.account
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_followed(&self) -> Option<bool> {
        self.is_followed
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn profile_image_urls(&self) -> &ImageUrl {
        &self.profile_image_urls
    }
}
