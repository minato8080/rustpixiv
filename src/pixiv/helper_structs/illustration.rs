use crate::enums::ContentType;

use crate::pixiv::helper_structs::image_url::ImageUrl;
use crate::pixiv::helper_structs::meta_page::MetaPage;
use crate::pixiv::helper_structs::series::Series;
use crate::pixiv::helper_structs::single_page_meta::SingleMetaPage;
use crate::pixiv::helper_structs::tag::Tag;
use crate::pixiv::result::illustration_proxy::IllustrationProxy;
use crate::pixiv::user::User;

use serde::{Deserialize, Serialize};

/// Struct representations of a PixivClient illustration.
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
    series: Option<Series>,
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

/// Convert `IllustrationProxy` to `Illustration`
impl From<IllustrationProxy> for Illustration {
    fn from(proxy: IllustrationProxy) -> Self {
        proxy.illust
    }
}

impl Illustration {
    pub fn download(&self, client: &reqwest::blocking::Client, path: &std::path::Path) {
        let urls: Vec<reqwest::Url> = self
            .image_urls
            .clone()
            .into_iter()
            .map(|url| reqwest::Url::parse(&url))
            .filter_map(Result::ok)
            .collect();

        for target in urls {
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
            let response = response.unwrap();
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

            let bytes = response.bytes().unwrap();
            let mut content = bytes.as_ref();
            std::io::copy(&mut content, &mut dest).unwrap();
        }
    }

    pub fn caption(&self) -> &String {
        &self.caption
    }

    pub fn create_date(&self) -> &String {
        &self.create_date
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn image_urls(&self) -> &ImageUrl {
        &self.image_urls
    }

    pub fn is_bookmarked(&self) -> bool {
        self.is_bookmarked
    }

    pub fn is_muted(&self) -> bool {
        self.is_muted
    }

    pub fn meta_pages(&self) -> &Vec<MetaPage> {
        &self.meta_pages
    }

    pub fn meta_single_page(&self) -> Option<&SingleMetaPage> {
        self.meta_single_page.as_ref()
    }

    pub fn page_count(&self) -> u32 {
        self.page_count
    }

    pub fn restrict(&self) -> u32 {
        self.restrict
    }

    pub fn sanity_level(&self) -> u32 {
        self.sanity_level
    }

    pub fn series(&self) -> Option<&Series> {
        self.series.as_ref()
    }

    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn tools(&self) -> &Vec<String> {
        &self.tools
    }

    pub fn total_bookmarks(&self) -> u32 {
        self.total_bookmarks
    }

    pub fn total_comments(&self) -> Option<u32> {
        self.total_comments
    }

    pub fn total_view(&self) -> u32 {
        self.total_view
    }

    pub fn content_type(&self) -> &ContentType {
        &self.content_type
    }

    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn x_restrict(&self) -> u32 {
        self.x_restrict
    }
}
