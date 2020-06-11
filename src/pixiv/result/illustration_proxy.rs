use crate::pixiv::helper_structs::illustration::Illustration;

use serde::{Deserialize, Serialize};

/// Convert `IllustrationProxy` to `Illustration`
impl From<IllustrationProxy> for Illustration {
    fn from(proxy: IllustrationProxy) -> Self {
        proxy.illust
    }
}

/// PixivClient hides the actual illustration object behind the value "illust".
/// This struct exists purely to bypass this indirection...
#[derive(Serialize, Deserialize, Debug)]
pub struct IllustrationProxy {
    illust: Illustration,
}

impl IllustrationProxy {
    pub fn into_inner(self) -> Illustration {
        self.illust
    }
}
