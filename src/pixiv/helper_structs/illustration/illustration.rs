use crate::enums::ContentType;
use crate::pixiv::client::Pixiv;
use crate::pixiv::helper_structs::illustration::meta_page::MetaPage;
use crate::pixiv::helper_structs::illustration::single_page_meta::SingleMetaPage;
use crate::pixiv::helper_structs::illustration::tag::Tag;
use crate::pixiv::helper_structs::image_url::ImageUrl;
use crate::pixiv::user::User;

use serde::{Deserialize, Serialize};

/// Struct representations of a Pixiv illustration.
/// TODO: We need to verify this struct, handles nullable types (possibly introduce a default value)
/// and maybe stuffs that are not known (i.e. we have not encountered that property)
#[derive(Deserialize, Serialize, Debug)]
pub struct Illustration {
    caption: String,
    create_date: String,
    height: u32,
    width: u32,
    id: u32,
    image_urls: ImageUrl,
    is_bookmarked: bool,
    is_muted: bool,
    meta_pages: Vec<MetaPage>,
    meta_single_page: Option<SingleMetaPage>,
    page_count: u32,
    restrict: u32,
    sanity_level: u32,
    series: Option<String>,
    tags: Vec<Tag>,
    title: String,
    tools: Vec<String>, // This should be an enum because we all the possible tools.
    total_bookmarks: u32,
    total_comments: Option<u32>,
    total_view: u32,
    #[serde(rename = "type")]
    content_type: ContentType, // This should be an enum
    // TODO: This should be borrowed?
    user: User,
    visible: bool,
    x_restrict: u32,
}

impl Pixiv {
    pub fn download_illustration<'a, 'b, 'c>(
        &'a self,
        path: &'b std::path::Path,
        illustration: &'c Illustration,
    ) {
        illustration
            .image_urls
            .clone()
            .into_iter()
            .map(|url| reqwest::Url::parse(&url))
            .filter_map(Result::ok)
            .for_each(|target: reqwest::Url| {
                let response = self
                    .client
                    .request(http::Method::GET, target)
                    .header(
                        reqwest::header::REFERER,
                        format!(
                            "https://www.pixiv.net/member_illust.php?mode=medium&illust_id={}",
                            illustration.id
                        ),
                    )
                    .send();
                let mut response = response.unwrap();
                let mut dest = {
                    println!("response_url:{}", response.url());
                    let fname = response
                        .url()
                        .path_segments()
                        .and_then(|segments| segments.last())
                        .and_then(|name| if name.is_empty() { None } else { Some(name) })
                        .unwrap_or("tmp.bin");
                    println!("file to download_illustration: '{}'", fname);
                    let fname = path.join(fname);
                    println!("will be located under: '{:?}'", fname);
                    std::fs::File::create(fname).unwrap()
                };
                std::io::copy(&mut response, &mut dest).unwrap();
            })
    }
}
