use crate::pixiv::helper_structs::illustration::Illustration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendedIllustration {
    contest_exists: bool,
    illusts: Vec<Illustration>,
    next_url: String,
    // TODO: Figure out the correct struct for this.
    privacy_policy: serde_json::Value,
    // TODO: Figure out the correct struct for this.
    ranking_illusts: Vec<String>,
}
