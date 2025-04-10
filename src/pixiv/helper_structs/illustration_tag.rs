use crate::pixiv::helper_structs::illustration::Illustration;
use serde::{Deserialize, Serialize};

/// IllustrationTag
#[derive(Serialize, Deserialize, Debug)]
pub struct IllustrationTag {
    illust: Illustration,
    tag: String,
    translate_name: Option<String>,
}

impl IllustrationTag {
    pub fn get_illust(&self) -> &Illustration {
        &self.illust
    }

    pub fn get_tag(&self) -> &String {
        &self.tag
    }

    pub fn get_translate_name(&self) -> &Option<String> {
        &self.translate_name
    }
}
