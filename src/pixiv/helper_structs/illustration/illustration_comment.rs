use crate::pixiv::helper_structs::illustration::comment::Comment;
use serde::{Deserialize, Serialize};

/// IllustrationComment
#[derive(Serialize, Deserialize, Debug)]
pub struct IllustrationComment {
    comments: Vec<Comment>,
    next_url: String,
    total_comments: u32,
}

impl IntoIterator for IllustrationComment {
    type Item = Comment;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    /// Consume the struct, yielding an iterator.
    fn into_iter(self) -> Self::IntoIter {
        self.comments.into_iter()
    }
}
