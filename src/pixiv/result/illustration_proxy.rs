use crate::pixiv::helper_structs::illustration::Illustration;

use serde::{Deserialize, Serialize};

/// PixivClient hides the actual illustration object behind the value "illust".
/// This struct exists purely to bypass this indirection...
#[derive(Serialize, Deserialize, Debug)]
pub struct IllustrationProxy {
    pub illust: Illustration,
}

impl IllustrationProxy {
    pub fn into_inner(self) -> Illustration {
        self.illust
    }
}
