use crate::enums::Visibility;

pub struct UserBookmarkTagsIllustrationRequestArg {
    pub restrict: Visibility,
    pub offset: Option<u32>,
}

impl Default for UserBookmarkTagsIllustrationRequestArg {
    fn default() -> Self {
        UserBookmarkTagsIllustrationRequestArg {
            restrict: Visibility::Public,
            offset: None,
        }
    }
}
