use crate::constants::BASE_URL;
use crate::enums::{Filter, RankingType, Visibility};
use crate::pixiv::helper_structs::illustration::illustration_search_param::IllustrationSearchParam;
use crate::pixiv::helper_structs::illustration::req_illust_ranking_arg::IllustRankingArg;
use crate::pixiv::helper_structs::illustration::req_rec_illust_arg::IllustRecArg;
use crate::pixiv::request::PixivRequest;
use crate::utils::comma_delimited;

use bytes::Bytes;
use http::{uri::Uri, HttpTryFrom, Method};
use std::borrow::Borrow;

/// PixivClient request builder. You can create this using any of the provided methods in `PixivClient`, or through `PixivRequestBuilder::new`.
#[derive(Debug, Clone)]
pub struct PixivRequestBuilder;

/// TODO: This should become a factory in the future...
impl PixivRequestBuilder {
    /// Used to build a request to retrive `bad_words.json`.
    /// # Request Transforms
    /// None
    pub fn request_bad_words() -> PixivRequest {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1.1/bad_words.json";
        let api_uri = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, api_uri)
    }

    /////////////////////////////////////////////////////////////////////
    /////
    /////                        VERSION 1 API
    /////
    /////////////////////////////////////////////////////////////////////

    pub fn work(illust_id: usize) -> PixivRequest {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/works/{}.json",
            illust_id
        );
        let url = Uri::try_from(url).unwrap();
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("image_sizes", "px_128x128,small,medium,large,px_480mw")
            .add_param_from_str("include_stats", "true")
            .finish()
    }

    pub fn user(user_id: usize) -> PixivRequest {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}.json",
            user_id
        );
        let url = Uri::try_from(&url).unwrap();
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("profile_image_sizes", "px_170x170,px_50x50")
            .add_param_from_str("image_sizes", "px_128x128,small,medium,large,px_480mw")
            .add_param_from_str("include_stats", "1")
            .add_param_from_str("include_profile", "1")
            .add_param_from_str("include_workspace", "1")
            .add_param_from_str("include_contacts", "1")
            .finish()
    }

    pub fn feed() -> PixivRequest {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/me/feeds.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("relation", "all")
            .add_param_from_str("type", "touch_nottext")
            .add_param_from_str("show_r18", "1")
            .finish()
    }

    pub fn favorite_works() -> PixivRequest {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite_works.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "50")
            .add_param_from_str("publicity", "public")
            .add_param_from_str("image_sizes", "px_128x128,px_480mw,large")
            .finish()
    }

    pub fn favorite_work_add(work_id: usize) -> PixivRequest {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite_works.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::POST, url)
            .add_param_from_str("publicity", "public")
            .add_param_from_str("work_id", work_id.to_string().as_str())
            .finish()
    }

    pub fn favorite_works_remove<B, I>(work_ids: I) -> PixivRequest
    where
        B: Borrow<usize>,
        I: IntoIterator<Item = B>,
    {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite_works.json";
        let url = Uri::from_static(API_URL);

        PixivRequest::new(Method::DELETE, url)
            .add_param_from_str("publicity", "public")
            .add_param_from_str("ids", comma_delimited(work_ids).as_str())
            .finish()
    }

    pub fn following_works(image_sizes: &[&str], include_sanity_level: bool) -> PixivRequest {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/following/works.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
            .add_param_from_str(
                "image_sizes",
                image_sizes
                    .into_iter()
                    .fold(String::new(), |acc, x| acc + format!(",{}", *x).as_str())
                    .as_str(),
            )
            .add_param_from_str("include_stats", "true")
            .add_param_from_str(
                "include_sanity_level",
                include_sanity_level.to_string().as_str(),
            )
            .finish()
    }

    pub fn request_illustration_search<T>(params: T) -> PixivRequest
    where
        T: Into<IllustrationSearchParam>,
    {
        let url = http::Uri::try_from(format!("{}/v1/search/illust", BASE_URL)).unwrap();
        params
            .into()
            .into_iter()
            .fold(
                PixivRequest::new(Method::GET, url),
                |acc, (k, v): (&'static str, String)| acc.add_param_from_str(k, v.as_str()),
            )
            .finish()
    }

    pub fn following() -> PixivRequest {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/me/following.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
            .add_param_from_str("publicity", "public")
            .finish()
    }

    pub fn following_add(user_id: usize) -> PixivRequest {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite-users.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::POST, url)
            .add_param_from_str("publicity", "public")
            .add_param_from_str("target_user_id", user_id.to_string().as_str())
            .finish()
    }

    pub fn following_remove<B, I>(user_ids: I) -> PixivRequest
    where
        B: Borrow<usize>,
        I: IntoIterator<Item = B>,
    {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite-users.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::DELETE, url)
            .add_param_from_str("publicity", "public")
            .add_param_from_str("delete_ids", comma_delimited(user_ids).as_str())
            .finish()
    }

    pub fn user_works(user_id: usize) -> PixivRequest {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/works.json",
            user_id
        );
        let url = Uri::try_from(&url).unwrap();
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
            .add_param_from_str("image_sizes", "px_128x128,px480mw,large")
            .add_param_from_str("include_stats", "true")
            .add_param_from_str("include_sanity_level", "true")
            .finish()
    }

    pub fn user_favorite_works(user_id: usize) -> PixivRequest {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/favorite_works.json",
            user_id
        );
        let url = Uri::try_from(&url).unwrap();
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
            .add_param_from_str("image_sizes", "px_128x128,px480mw,large")
            .add_param_from_str("include_sanity_level", "true")
            .finish()
    }

    pub fn user_feed(user_id: usize) -> PixivRequest {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/feeds.json",
            user_id
        );
        let url = Uri::try_from(&url).unwrap();
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("relation", "all")
            .add_param_from_str("type", "touch_nottext")
            .add_param_from_str("show_r18", "1")
            .finish()
    }

    pub fn user_following(user_id: usize) -> PixivRequest {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/following.json",
            user_id
        );
        let url = Uri::try_from(&url).unwrap();
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
    }

    pub fn ranking(ranking_type: RankingType) -> PixivRequest {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/ranking/{}.json",
            ranking_type.as_str()
        );
        let url = Uri::try_from(&url).unwrap();
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("mode", "daily")
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "50")
            .add_param_from_str("include_stats", "True")
            .add_param_from_str("include_sanity_level", "True")
            .add_param_from_str("image_sizes", "px_128x128,small,medium,large,px_480mw")
            .add_param_from_str("profile_image_sizes", "px_170x170,px_50x50")
            .finish()
    }

    pub fn search_works<T>(query: T) -> PixivRequest
    where
        T: Into<String>,
    {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/search/works.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
            .add_param_from_str("mode", "text")
            .add_param_from_str("period", "all")
            .add_param_from_str("order", "desc")
            .add_param_from_str("sort", "date")
            .add_param_from_str("types", "illustration,manga,ugoira")
            .add_param_from_str("include_stats", "true")
            .add_param_from_str("include_sanity_level", "true")
            .add_param_from_str("image_sizes", "px_128x128,px480mw,large")
            .add_param_from_str("q", query.into().as_str())
            .finish()
    }

    pub fn latest_works() -> PixivRequest {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/works.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
            .add_param_from_str("include_stats", "true")
            .add_param_from_str("include_sanity_level", "true")
            .add_param_from_str("image_sizes", "px_128x128,px480mw,large")
            .add_param_from_str("profile_image_sizes", "px_170x170,px_50x50")
            .finish()
    }

    /// Used to build a request to fetch an illustration givenn its id.
    pub fn request_illustration(illust_id: usize) -> PixivRequest {
        let uri = format!("{}/v1/illust/detail", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param_from_str("illust_id", illust_id.to_string().as_str())
            .finish()
    }

    /// Used to build a request to fetch an illustration givenn its id.
    pub fn request_illustration_comments(
        illust_id: usize,
        offset: usize,
        include_total_comments: bool,
    ) -> PixivRequest {
        let uri = format!("{}/v1/illust/comments", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param_from_str("illust_id", illust_id.to_string().as_str())
            .add_param_from_str("offset", offset.to_string().as_str())
            .add_param_from_str(
                "include_total_comments",
                include_total_comments.to_string().as_str(),
            )
            .finish()
    }

    /// TODO: Documentation
    pub fn request_recommended_illustration<T>(argument: T) -> PixivRequest
    where
        T: Into<IllustRecArg>,
    {
        let argument = argument.into();
        let uri = format!("{}/v1/illust/recommended", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        argument
            .build()
            .iter()
            .fold(PixivRequest::new(Method::GET, uri), |acc, (key, val)| {
                acc.add_param_from_str(key, val)
            })
            .finish()
    }

    /// TODO: Documentation
    pub fn request_illustration_ranking<T>(argument: T) -> PixivRequest
    where
        T: Into<IllustRankingArg>,
    {
        let argument = argument.into();
        let uri = format!("{}/v1/illust/ranking", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        argument
            .build()
            .iter()
            .fold(PixivRequest::new(Method::GET, uri), |acc, (key, val)| {
                acc.add_param_from_str(key, val)
            })
            .finish()
    }

    /// TODO: Documentation
    pub fn request_trending_tags() -> PixivRequest {
        let uri = format!("{}/v1/trending-tags/illust", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param_from_str("filter", Filter::ForiOS.as_str())
            .finish()
    }

    /////////////////////////////////////////////////////////////////////
    /////
    /////                        VERSION 2 API
    /////
    /////////////////////////////////////////////////////////////////////

    /// TODO: Documentation
    pub fn request_related_illustration(illust_id: usize, offset: usize) -> PixivRequest {
        let uri = format!("{}/v2/illust/related", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param_from_str("illust_id", illust_id.to_string().as_str())
            .add_param_from_str("offset", offset.to_string().as_str())
            .finish()
    }

    /// TODO: Documentation
    pub fn request_illustration_following<T>(visibility: T) -> PixivRequest
    where
        T: Into<Visibility>,
    {
        let uri = format!("{}/v2/illust/follow", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param_from_str("restrict", visibility.into().as_str())
            .finish()
    }

    /// TODO: Documentation
    pub fn request_illustration_bookmark_info(illust_id: usize) -> PixivRequest {
        let uri = format!("{}/v2/illust/bookmark/detail", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param_from_str("illust_id", illust_id.to_string().as_str())
            .finish()
    }
}
