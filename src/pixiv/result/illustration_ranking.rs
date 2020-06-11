use crate::pixiv::helper_structs::illustration::Illustration;

use serde::{Deserialize, Serialize};

/// PixivClient hides the actual illustration object behind the value "illust".
/// This struct exists purely to bypass this indirection...
#[derive(Serialize, Deserialize, Debug)]
pub struct IllustrationRanking {
    illusts: Vec<Illustration>,
    next_url: String,
}
