use crate::pixiv::helper_structs::illustration::illustration::Illustration;
use serde::{Deserialize, Serialize};

/// IllustrationTag
#[derive(Serialize, Deserialize, Debug)]
pub struct IllustrationTag {
    illust: Illustration,
    tag: String,
    translate_name: Option<String>,
}
