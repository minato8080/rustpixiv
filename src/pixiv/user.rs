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
