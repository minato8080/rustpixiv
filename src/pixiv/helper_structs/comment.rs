use crate::pixiv::user::User;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    comment: String,
    date: String,
    id: u64,
    // TODO: Ignored for now...
    // parent_comment :
    user: User,
}
