use crate::pixiv::helper_structs::illustration_tag::IllustrationTag;
use serde::{Deserialize, Serialize};

/// TrendingIllustration
#[derive(Serialize, Deserialize, Debug)]
pub struct TrendingIllustrations {
    trend_tags: Vec<IllustrationTag>,
}
