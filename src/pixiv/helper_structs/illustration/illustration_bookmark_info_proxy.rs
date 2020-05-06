use crate::pixiv::helper_structs::illustration::illustration_bookmark_info::IllustBookmarkInfo;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IllustBookmarkInfoProxy {
    bookmark_detail: IllustBookmarkInfo,
}
