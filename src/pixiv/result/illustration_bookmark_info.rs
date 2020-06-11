use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IllustBookmarkInfoTag {
    is_registered: bool,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IllustBookmarkInfo {
    is_bookmarked: bool,
    restrict: String,
    tags: Vec<IllustBookmarkInfoTag>,
}
