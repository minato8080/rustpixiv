use crate::constants::{BASE_URL, ILLUST_ID, OFFSET, RESTRICT, TAGS, USER_ID};
use crate::enums::{Filter, RankingType, Visibility};
use crate::pixiv::arg::illustration_ranking_request_arg::IllustrationRankingRequestArg;
use crate::pixiv::arg::illustration_search_request_arg::IllustrationSearchRequestArg;
use crate::pixiv::arg::recommended_illustration_request_arg::RecommendedIllustrationRequestArg;
use crate::pixiv::arg::user_bookmark_tags_illustration_request_arg::UserBookmarkTagsIllustrationRequestArg;
use crate::pixiv::arg::user_following_request_arg::UserFollowingRequestArgs;
use crate::pixiv::request::PixivRequest;
use crate::utils::comma_delimited;

use http::{uri::Uri, Method};
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
            .add_param("work_id", work_id.to_string())
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
            .add_param("ids", comma_delimited(work_ids))
            .finish()
    }

    pub fn following_works(image_sizes: &[&str], include_sanity_level: bool) -> PixivRequest {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/following/works.json";
        let url = Uri::from_static(API_URL);
        PixivRequest::new(Method::GET, url)
            .add_param_from_str("page", "1")
            .add_param_from_str("per_page", "30")
            .add_param(
                "image_sizes",
                image_sizes
                    .into_iter()
                    .fold(String::new(), |acc, x| acc + format!(",{}", *x).as_str()),
            )
            .add_param_from_str("include_stats", "true")
            .add_param("include_sanity_level", include_sanity_level.to_string())
            .finish()
    }

    pub fn request_illustration_search<T>(params: T) -> PixivRequest
    where
        T: Into<IllustrationSearchRequestArg>,
    {
        let url = http::Uri::try_from(format!("{}/v1/search/illust", BASE_URL)).unwrap();
        params
            .into()
            .into_iter()
            .fold(
                PixivRequest::new(Method::GET, url),
                |acc, (k, v): (&'static str, String)| acc.add_param(k, v),
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
            .add_param("target_user_id", user_id.to_string())
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
            .add_param("delete_ids", comma_delimited(user_ids).as_str())
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
            .add_param("q", query.into().as_str())
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
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param(ILLUST_ID, illust_id.to_string())
            .finish()
    }

    /// Used to build a request to fetch an illustration givenn its id.
    pub fn request_illustration_comments(
        illust_id: usize,
        offset: usize,
        include_total_comments: bool,
    ) -> PixivRequest {
        let uri = format!("{}/v1/illust/comments", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param(ILLUST_ID, illust_id.to_string())
            .add_param(OFFSET, offset.to_string())
            .add_param("include_total_comments", include_total_comments.to_string())
            .finish()
    }

    /// TODO: Documentation
    pub fn request_recommended_illustration<T>(argument: T) -> PixivRequest
    where
        T: Into<RecommendedIllustrationRequestArg>,
    {
        let argument = argument.into();
        let uri = format!("{}/v1/illust/recommended", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        argument
            .build()
            .iter()
            .fold(PixivRequest::new(Method::GET, uri), |acc, (key, val)| {
                acc.add_param(key, String::from(val))
            })
            .finish()
    }

    /// TODO: Documentation
    pub fn request_illustrations_ranking<T>(argument: T) -> PixivRequest
    where
        T: Into<IllustrationRankingRequestArg>,
    {
        let argument = argument.into();
        let uri = format!("{}/v1/illust/ranking", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        argument
            .build()
            .iter()
            .fold(PixivRequest::new(Method::GET, uri), |acc, (key, val)| {
                acc.add_param(key, String::from(val))
            })
            .finish()
    }

    /// TODO: Documentation
    pub fn request_trending_tags() -> PixivRequest {
        let uri = format!("{}/v1/trending-tags/illust", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
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
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param(ILLUST_ID, illust_id.to_string())
            .add_param(OFFSET, offset.to_string())
            .finish()
    }

    /// TODO: Documentation
    pub fn request_illustration_following<T>(visibility: T) -> PixivRequest
    where
        T: Into<Visibility>,
    {
        let uri = format!("{}/v2/illust/follow", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param_from_str(RESTRICT, visibility.into().as_str())
            .finish()
    }

    /// TODO: Documentation
    pub fn request_illustration_bookmark_info(illust_id: usize) -> PixivRequest {
        let uri = format!("{}/v2/illust/bookmark/detail", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param(ILLUST_ID, illust_id.to_string())
            .finish()
    }

    /// TODO: Documentation
    pub fn request_adding_bookmark(illust_id: usize, visibility: Visibility) -> PixivRequest {
        let uri = format!("{}/v2/illust/bookmark/add", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::POST, uri)
            .add_form(ILLUST_ID, illust_id.to_string().as_str())
            .add_form_from_str(RESTRICT, visibility.as_str())
            .add_form_from_str(TAGS, "Fate/GO")
            .finish()
    }

    /// TODO: Documentation
    /// TODO: This is V1 for whatever reason...
    pub fn request_delete_bookmark(illust_id: usize) -> PixivRequest {
        let uri = format!("{}/v1/illust/bookmark/delete", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::POST, uri)
            .add_form(ILLUST_ID, illust_id.to_string())
            .finish()
    }

    /// TODO: Documentation
    /// TODO: Test
    pub fn request_user_bookmark_tags_illustration(
        args: UserBookmarkTagsIllustrationRequestArg,
    ) -> PixivRequest {
        let uri = format!("{}/v1/user/bookmark-tags/illust", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param(RESTRICT, args.restrict.as_str())
            .maybe_add_param(OFFSET, args.offset.map(|x| x.to_string()))
            .finish()
    }

    /// TODO: Documentation
    /// TODO: Test
    pub fn request_user_following(args: UserFollowingRequestArgs) -> PixivRequest {
        let uri = format!("{}/v1/user/following", BASE_URL);
        let uri = Uri::try_from(uri.as_str()).unwrap();
        PixivRequest::new(Method::GET, uri)
            .add_param(USER_ID, args.user_id.to_string())
            .add_param(RESTRICT, args.restrict.as_str())
            .add_param(OFFSET, args.offset.to_string())
            .finish()
    }
}
