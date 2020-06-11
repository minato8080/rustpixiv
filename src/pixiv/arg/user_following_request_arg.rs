use crate::enums::Visibility;

pub struct UserFollowingRequestArgs {
    pub user_id: u32,
    pub restrict: Visibility,
    pub offset: u32,
}

impl UserFollowingRequestArgs {
    pub fn new(user_id: u32) -> Self {
        UserFollowingRequestArgs {
            user_id,
            restrict: Visibility::Public,
            offset: 0u32,
        }
    }
}
