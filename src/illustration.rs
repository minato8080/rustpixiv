use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ImageUrl {
    pub large: Option<String>,
    pub medium: Option<String>,
    pub small: Option<String>,
    pub square_medium: Option<String>,
}

impl IntoIterator for ImageUrl {
    type Item = String;
    type IntoIter = ImageUrlIterator;

    fn into_iter(self) -> Self::IntoIter {
        ImageUrlIterator {
            url: self,
            index: 0,
        }
    }
}

pub struct ImageUrlIterator {
    url: ImageUrl,
    index: usize,
}

impl Iterator for ImageUrlIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let result = match self.index {
                0 => self.url.small.take(),
                1 => self.url.medium.take(),
                2 => self.url.large.take(),
                3 => self.url.square_medium.take(),
                _ => return None,
            };
            self.index += 1;

            if let Some(r) = result {
                return Some(r);
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct SinglePageMeta {
    original_image_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Tag {
    name: String,
    translated_name: Option<Vec<String>>,
}

/// The user who worked on the illustration (the artist).
#[derive(Deserialize, Serialize, Debug)]
struct User {
    account: String,
    id: u32,
    is_followed: bool,
    name: String,
    profile_image_urls: ImageUrl,
}

#[derive(Deserialize, Serialize, Debug)]
enum ContentType {
    #[serde(rename = "illust")]
    Illustration,
    #[serde(rename = "manga")]
    Manga,
    #[serde(rename = "ugoira")]
    Ugoira,
    #[serde(rename = "novel")]
    Novel,
}

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
    types: ContentType, // This should be an enum
    user: User,
    visible: bool,
    x_restrict: u32,
}

impl Illustration {
    pub fn download(&self, client: &reqwest::Client) {
        let path = std::env::current_dir().unwrap();
        self.image_urls
            .clone()
            .into_iter()
            .map(|str| reqwest::Url::parse(&str))
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
