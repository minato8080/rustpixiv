use crate::constants::BASE_URL;
use crate::enums::ContentType;
use crate::pixiv::helper_structs::image_url::ImageUrl;
use crate::pixiv::illustration::illustration_search_param::IllustrationSeachParam;
use crate::pixiv::illustration::single_page_meta::SinglePageMeta;
use crate::pixiv::illustration::tag::Tag;
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
    total_view: u32,
    #[serde(rename = "type")]
    content_type: ContentType, // This should be an enum
    // TODO: This should be borrowed?
    user: User,
    visible: bool,
    x_restrict: u32,
}

impl Illustration {
    pub fn search(&self, client: &reqwest::Client, _params: IllustrationSeachParam) {
        let target = reqwest::Url::parse(&format!("{}/v1/search/illust", BASE_URL)).unwrap();
        let _response = client
            .request(http::Method::GET, target)
            .header(
                reqwest::header::REFERER,
                format!(
                    "https://www.pixiv.net/member_illust.php?mode=medium&illust_id={}",
                    self.id
                ),
            )
            .send();
    }

    pub fn download(&self, client: &reqwest::Client, path: &std::path::Path) {
        self.image_urls
            .clone()
            .into_iter()
            .map(|url| reqwest::Url::parse(&url))
            .filter_map(Result::ok)
            .for_each(|target: reqwest::Url| {
                let response = client
                    .request(http::Method::GET, target)
                    .header(
                        reqwest::header::REFERER,
                        format!(
                            "https://www.pixiv.net/member_illust.php?mode=medium&illust_id={}",
                            self.id
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
                    println!("file to download: '{}'", fname);
                    let fname = path.join(fname);
                    println!("will be located under: '{:?}'", fname);
                    std::fs::File::create(fname).unwrap()
                };
                std::io::copy(&mut response, &mut dest).unwrap();
            })
    }
}

/// Convert `IllustrationProxy` to `Illustration`
impl From<IllustrationProxy> for Illustration {
    fn from(proxy: IllustrationProxy) -> Self {
        proxy.illust
    }
}

/// Pixiv hides the actual illustration object behind the value "illust".
/// This struct exists purely to bypass this indirection...
#[derive(Serialize, Debug, Deserialize)]
pub struct IllustrationProxy {
    illust: Illustration,
}

impl IllustrationProxy {
    pub fn into_inner(self) -> Illustration {
        self.illust
    }
}
