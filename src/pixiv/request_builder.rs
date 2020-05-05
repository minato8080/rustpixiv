use crate::constants::BASE_URL;
use crate::enums::Visibility;
use crate::enums::{Publicity, RankingMode, RankingType, SearchMode, SearchOrder, SearchPeriod};
use crate::pixiv::helper_structs::illustration::illustration_search_param::IllustrationSearchParam;
use crate::pixiv::helper_structs::illustration::req_illust_ranking_arg::IllustRankingArg;
use crate::pixiv::helper_structs::illustration::req_rec_illust_arg::IllustRecArg;
use crate::pixiv::request::PixivRequest;
use crate::utils::comma_delimited;

use bytes::Bytes;
use chrono::naive::NaiveDate;
use http::{header, uri::Uri, HeaderMap, HttpTryFrom, Method};
use std::borrow::Borrow;
use std::collections::HashMap;

/// PixivClient request builder. You can create this using any of the provided methods in `PixivClient`, or through `PixivRequestBuilder::new`.
#[derive(Debug, Clone)]
pub struct PixivRequestBuilder {
    request: PixivRequest,
    params: HashMap<&'static str, String>,
}

impl PixivRequestBuilder {
    /// Create a new `PixivRequestBuilder`.
    /// Functions in `PixivClient` expedite a lot of this for you, so using this directly isn't recommended unless you know what you want.
    pub fn new(method: Method, url: Uri, params: HashMap<&'static str, String>) -> Self {
        // set headers
        let mut headers = HeaderMap::new();
        headers.insert(
            header::REFERER,
            header::HeaderValue::from_static("http://spapi.pixiv.net/"),
        );

        PixivRequestBuilder {
            request: PixivRequest::new(method, url, headers),
            params,
        }
    }
    /// Sets the `page` param.

    pub fn page(self, value: usize) -> Self {
        self.raw_param("page", value.to_string())
    }
    /// Sets the `per_page` param.

    pub fn per_page(self, value: usize) -> Self {
        self.raw_param("value", value.to_string())
    }
    /// Sets the `max_id` param.

    pub fn max_id(self, value: usize) -> Self {
        self.raw_param("max_id", value.to_string())
    }
    /// Sets the `image_sizes` param. Available types: `px_128x128`, `small`, `medium`, `large`, `px_480mw`

    pub fn image_sizes(self, values: &[&str]) -> Self {
        self.raw_param("image_sizes", comma_delimited::<&str, _, _>(values))
    }
    /// Sets the `profile_image_sizes` param. Available types: `px_170x170,px_50x50`

    pub fn profile_image_sizes(self, values: &[&str]) -> Self {
        self.raw_param("profile_image_sizes", comma_delimited::<&str, _, _>(values))
    }
    /// Sets the `publicity` param. Must be a value of enum `Publicity`.

    pub fn publicity(self, value: Publicity) -> Self {
        self.raw_param("publicity", value.as_str())
    }
    /// Sets the `show_r18` param. `true` means R-18 works will be included.

    pub fn show_r18(self, value: bool) -> Self {
        if value {
            self.raw_param("show_r18", "1")
        } else {
            self.raw_param("show_r18", "0")
        }
    }
    /// Sets the `include_stats` param.

    pub fn include_stats(self, value: bool) -> Self {
        if value {
            self.raw_param("include_stats", "true")
        } else {
            self.raw_param("include_stats", "false")
        }
    }

    /// Sets the `include_sanity_level` param.
    pub fn include_sanity_level(self, value: bool) -> Self {
        if value {
            self.raw_param("include_sanity_level", "true")
        } else {
            self.raw_param("include_sanity_level", "false")
        }
    }

    /// Sets the ranking mode in the case of a `ranking()` call. Must be a value of enum `RankingMode`.
    pub fn ranking_mode(self, value: RankingMode) -> Self {
        self.raw_param("mode", value.as_str())
    }

    /// Sets the `date` param. Must be a valid date in the form of `%Y-%m-%d`, e.g. `2018-2-22`.
    pub fn date<V>(self, value: V) -> Self
    where
        V: Into<String>,
    {
        let value = value.into();
        // just to validate the date format
        NaiveDate::parse_from_str(&*value, "%Y-%m-%d").expect("Invalid date or format given.");
        self.raw_param("date", value)
    }

    /// Sets the `period` param in the case of a `search_works()` call. Must be a value of enum `SearchPeriod`.
    pub fn search_period(self, value: SearchPeriod) -> Self {
        self.raw_param("period", value.as_str())
    }

    /// Sets the `mode` param in the case of a `search_works()` call. Must be a value of enum `SearchMode`.
    pub fn search_mode(self, value: SearchMode) -> Self {
        self.raw_param("mode", value.as_str())
    }

    /// Sets the `order` param in the case of a `search_works()` call. Must be a value of enum `SearchOrder`.
    pub fn search_order(self, value: SearchOrder) -> Self {
        self.raw_param("order", value.as_str())
    }

    /// Sets the `sort` param in the case of a `search_works()` call. Not sure if there's any variations here, but this function is included for convenience.
    pub fn search_sort<V>(self, value: V) -> Self
    where
        V: Into<String>,
    {
        self.raw_param("sort", value)
    }

    /// Sets the `types` param in the case of a `search_works()` call. Available values: `illustration`, `manga`, `ugoira`.
    pub fn search_types(self, values: &[&str]) -> Self {
        self.raw_param("types", comma_delimited::<&str, _, _>(values))
    }

    // TODO: Add documentation.
    fn raw_param<V>(mut self, key: &'static str, value: V) -> Self
    where
        V: Into<String>,
    {
        self.params.insert(key, value.into());
        self
    }

    /// Used to build a request to retrive `bad_words.json`.
    /// # Request Transforms
    /// None
    pub fn bad_words() -> Self {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1.1/bad_words.json";
        let url = Uri::from_static(API_URL);
        PixivRequestBuilder::new(Method::GET, url, HashMap::default())
    }

    /////////////////////////////////////////////////////////////////////
    /////
    /////                        VERSION 2 API
    /////
    /////////////////////////////////////////////////////////////////////

    pub fn work(illust_id: usize) -> Self {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/works/{}.json",
            illust_id
        );
        let extra_params = [
            ("image_sizes", "px_128x128,small,medium,large,px_480mw"),
            ("include_stats", "true"),
        ];
        let url = Uri::try_from(url).unwrap();
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn user(user_id: usize) -> Self {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}.json",
            user_id
        );
        let extra_params = [
            ("profile_image_sizes", "px_170x170,px_50x50"),
            ("image_sizes", "px_128x128,small,medium,large,px_480mw"),
            ("include_stats", "1"),
            ("include_profile", "1"),
            ("include_workspace", "1"),
            ("include_contacts", "1"),
        ];
        let url = Uri::try_from(&url).unwrap();
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn feed() -> Self {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/me/feeds.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [
            ("relation", "all"),
            ("type", "touch_nottext"),
            ("show_r18", "1"),
        ];
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn favorite_works() -> Self {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite_works.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [
            ("page", "1"),
            ("per_page", "50"),
            ("publicity", "public"),
            ("image_sizes", "px_128x128,px_480mw,large"),
        ];
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn favorite_work_add(work_id: usize) -> Self {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite_works.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [("publicity", "public")];
        let params = extra_params
            .iter()
            .map(|&(k, v)| (k, v.into()))
            .chain(Some(("work_id", work_id.to_string().into())))
            .collect();
        PixivRequestBuilder::new(Method::POST, url, params)
    }

    pub fn favorite_works_remove<B, I>(work_ids: I) -> Self
    where
        B: Borrow<usize>,
        I: IntoIterator<Item = B>,
    {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite_works.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [("publicity", "public")];
        let params = extra_params
            .iter()
            .map(|&(k, v)| (k, v.into()))
            .chain(Some(("ids", comma_delimited(work_ids).into())))
            .collect();
        PixivRequestBuilder::new(Method::DELETE, url, params)
    }

    pub fn following_works() -> Self {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/following/works.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [
            ("page", "1"),
            ("per_page", "30"),
            ("image_sizes", "px_128x128,px480mw,large"),
            ("include_stats", "true"),
            ("include_sanity_level", "true"),
        ];
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn request_illustration_search<T>(params: T) -> Self
    where
        T: Into<IllustrationSearchParam>,
    {
        let url = http::Uri::try_from(format!("{}/v1/search/illust", BASE_URL)).unwrap();
        println!("url:{:#?}", url);
        let params = params
            .into()
            .into_iter()
            .map(|(k, v)| {
                println!("k:{} v:{}", k, v);
                (k, v)
            })
            .collect();
        println!("params:{:#?}", params);
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn following() -> Self {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/me/following.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [("page", "1"), ("per_page", "30"), ("publicity", "public")];
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn following_add(user_id: usize) -> Self {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite-users.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [("publicity", "public")];
        let params = extra_params
            .iter()
            .map(|&(k, v)| (k, v.into()))
            .chain(Some(("target_user_id", user_id.to_string().into())))
            .collect();
        PixivRequestBuilder::new(Method::POST, url, params)
    }

    pub fn following_remove<B, I>(user_ids: I) -> Self
    where
        B: Borrow<usize>,
        I: IntoIterator<Item = B>,
    {
        const API_URL: &'static str =
            "https://public-api.secure.pixiv.net/v1/me/favorite-users.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [("publicity", "public")];
        let params = extra_params
            .iter()
            .map(|&(k, v)| (k, v.into()))
            .chain(Some(("delete_ids", comma_delimited(user_ids).into())))
            .collect();
        PixivRequestBuilder::new(Method::DELETE, url, params)
    }

    pub fn user_works(user_id: usize) -> Self {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/works.json",
            user_id
        );
        let extra_params = [
            ("page", "1"),
            ("per_page", "30"),
            ("image_sizes", "px_128x128,px480mw,large"),
            ("include_stats", "true"),
            ("include_sanity_level", "true"),
        ];
        let url = Uri::try_from(&url).unwrap();
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn user_favorite_works(user_id: usize) -> Self {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/favorite_works.json",
            user_id
        );
        let extra_params = [
            ("page", "1"),
            ("per_page", "30"),
            ("image_sizes", "px_128x128,px480mw,large"),
            ("include_sanity_level", "true"),
        ];
        let url = Uri::try_from(&url).unwrap();
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn user_feed(user_id: usize) -> Self {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/feeds.json",
            user_id
        );
        let extra_params = [
            ("relation", "all"),
            ("type", "touch_nottext"),
            ("show_r18", "1"),
        ];
        let url = Uri::try_from(&url).unwrap();
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn user_following(user_id: usize) -> Self {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/users/{}/following.json",
            user_id
        );
        let extra_params = [("page", "1"), ("per_page", "30")];
        let url = Uri::try_from(&url).unwrap();
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn ranking(ranking_type: RankingType) -> Self {
        let url = format!(
            "https://public-api.secure.pixiv.net/v1/ranking/{}.json",
            ranking_type.as_str()
        );
        let extra_params = [
            ("mode", "daily"),
            ("page", "1"),
            ("per_page", "50"),
            ("include_stats", "True"),
            ("include_sanity_level", "True"),
            ("image_sizes", "px_128x128,small,medium,large,px_480mw"),
            ("profile_image_sizes", "px_170x170,px_50x50"),
        ];
        let url = Uri::try_from(&url).unwrap();
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn search_works<V>(query: V) -> PixivRequestBuilder
    where
        V: Into<String>,
    {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/search/works.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [
            ("page", "1"),
            ("per_page", "30"),
            ("mode", "text"),
            ("period", "all"),
            ("order", "desc"),
            ("sort", "date"),
            ("types", "illustration,manga,ugoira"),
            ("include_stats", "true"),
            ("include_sanity_level", "true"),
            ("image_sizes", "px_128x128,px480mw,large"),
        ];
        let params = extra_params
            .iter()
            .map(|&(k, v)| (k, v.into()))
            .chain(Some(("q", query.into())))
            .collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    pub fn latest_works() -> Self {
        const API_URL: &'static str = "https://public-api.secure.pixiv.net/v1/works.json";
        let url = Uri::from_static(API_URL);

        let extra_params = [
            ("page", "1"),
            ("per_page", "30"),
            ("include_stats", "true"),
            ("include_sanity_level", "true"),
            ("image_sizes", "px_128x128,px480mw,large"),
            ("profile_image_sizes", "px_170x170,px_50x50"),
        ];
        let params = extra_params.iter().map(|&(k, v)| (k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, url, params)
    }

    /// Used to build a request to fetch an illustration givenn its id.
    pub fn request_illustration(illust_id: usize) -> Self {
        let uri = format!("{}/v1/illust/detail", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        let extra_params = [("illust_id", illust_id.to_string())];
        let params = extra_params.iter().map(|(k, v)| (*k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, uri, params)
    }

    /// Used to build a request to fetch an illustration givenn its id.
    pub fn request_illustration_comments(
        illust_id: usize,
        offset: usize,
        include_total_comments: bool,
    ) -> Self {
        let uri = format!("{}/v1/illust/comments", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        let extra_params = [
            ("illust_id", illust_id.to_string()),
            ("offset", offset.to_string()),
            ("include_total_comments", include_total_comments.to_string()),
        ];
        let params = extra_params.iter().map(|(k, v)| (*k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, uri, params)
    }

    /// TODO: Documentation
    pub fn request_recommended_illustration<T>(argument: T) -> Self
    where
        T: Into<IllustRecArg>,
    {
        let argument = argument.into();
        let uri = format!("{}/v1/illust/recommended", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        let params = argument.build();
        PixivRequestBuilder::new(Method::GET, uri, params)
    }

    /// TODO: Documentation
    pub fn request_illustration_ranking<T>(argument: T) -> Self
    where
        T: Into<IllustRankingArg>,
    {
        let argument = argument.into();
        let uri = format!("{}/v1/illust/ranking", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        let params = argument.build();
        PixivRequestBuilder::new(Method::GET, uri, params)
    }

    /////////////////////////////////////////////////////////////////////
    /////
    /////                        VERSION 2 API
    /////
    /////////////////////////////////////////////////////////////////////

    /// TODO: Documentation
    pub fn request_related_illustration(illust_id: usize, offset: usize) -> Self {
        let uri = format!("{}/v2/illust/related", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        let extra_params = [
            ("illust_id", illust_id.to_string()),
            ("offset", offset.to_string()),
        ];
        let params = extra_params.iter().map(|(k, v)| (*k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, uri, params)
    }

    /// TODO: Documentation
    pub fn request_illustration_following<T>(visibility: T) -> Self
    where
        T: Into<Visibility>,
    {
        let uri = format!("{}/v2/illust/follow", BASE_URL);
        let bytes = Bytes::from(uri.as_str());
        let uri = Uri::from_shared(bytes).unwrap();
        let extra_params = [("restrict", visibility.into().as_str().to_string())];
        let params = extra_params.iter().map(|(k, v)| (*k, v.into())).collect();
        PixivRequestBuilder::new(Method::GET, uri, params)
    }

    /// Returns a `PixivRequest` which can be inspected and/or executed with `PixivClient::execute_with_auth()`.

    pub fn build(self) -> PixivRequest {
        self.request.set_query_params(&self.params)
    }
}
