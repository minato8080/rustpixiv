use serde::{Deserialize, Serialize};




#[derive(Deserialize, Serialize)]
struct ImageUrl {
    large: Option<String>,
    medium: Option<String>,
    small: Option<String>,
    square_medium: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct SinglePageMeta {
    original_image_url: String,
}

#[derive(Deserialize, Serialize)]
struct Tag {
    name: String,
    translated_name: Option<Vec<String>>,
}

/// The user who worked on the illustration (the artist).
#[derive(Deserialize, Serialize)]
struct User {
    account: String,
    id: u32,
    is_followed: bool,
    name: String,
    profile_image_urls: ImageUrl,
}

#[derive(Deserialize, Serialize)]
enum ContentType {
    Illustration,
    Manga,
    Ugoira,
    Novel,
}

#[derive(Deserialize, Serialize)]
pub struct Illustration {
    caption: String,
    create_data: String,
    height: u32,
    width: u32,
    id: u32,
    image_urls: ImageUrl,
    is_bookmarked: bool,
    is_muted: bool,
    meta_pages: Vec<String>,
    meta_single_page: SinglePageMeta,
    page_count: u32,
    restrict: u32,
    sanity_level: u32,
    series: Option<String>,
    tags: Vec<Tag>,
    title: String,
    tools: Vec<String>, // This should be an enum because we all the possible tools.
    total_bookmarks: u32,
    total_comments: u32,
    total_views: u32,
    types: ContentType, // This should be an enum
    user: User,
    visible: bool,
    x_restrict: u32,
}
