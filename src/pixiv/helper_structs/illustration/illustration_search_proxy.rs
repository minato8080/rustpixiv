use crate::pixiv::helper_structs::illustration::illustration::Illustration;

use serde::{Deserialize, Serialize};

/// Pixiv hides the actual illustration object behind the value "illust".
/// This struct exists purely to bypass this indirection...
#[derive(Serialize, Deserialize, Debug)]
pub struct IllustrationSearchProxy {
    illusts: Vec<Illustration>,
    next_url: String,
    search_span_limit: i32,
}

impl IllustrationSearchProxy {
    pub fn into_inner(self) -> Vec<Illustration> {
        self.illusts
    }
}
