use crate::utils::BytesWriter;

use http::{uri::Uri, HeaderMap, Method};

use std::io::Write;

/// PixivClient request. You can create this using `PixivRequestBuilder::build`. This is for if you wish to inspect the request before sending.
#[derive(Debug, Clone)]
pub struct PixivRequest {
    pub method: Method,
    pub url: Uri,
    pub headers: HeaderMap,
    pub params: std::collections::HashMap<&'static str, String>,
    pub form: std::collections::HashMap<&'static str, String>,
}

/// TODO: Consider replacing all the variations of adding stuff to a Trait
impl PixivRequest {
    /// Create a new `PixivRequest`.
    /// A `PixivRequest` is returned when calling `build()` on `PixivRequestBuilder`, so it is recommended you use that instead.
    pub fn new(method: Method, url: Uri) -> PixivRequest {
        let result = PixivRequest {
            method,
            url: url,
            headers: HeaderMap::new(),
            params: std::collections::HashMap::new(),
            form: std::collections::HashMap::new(),
        };
        result.add_header(
            http::header::REFERER,
            http::header::HeaderValue::from_static("http://spapi.pixiv.net/"),
        )
    }

    /// Add header to the request
    pub fn add_header<T, U>(mut self, key: T, val: U) -> Self
    where
        T: Into<http::header::HeaderName>,
        U: Into<http::header::HeaderValue>,
    {
        self.headers.insert(key.into(), val.into());
        self
    }

    /// Add header to the request
    /// TODO: Propagate this error.
    pub fn add_header_str<'a, T>(mut self, key: T, val: &'a str) -> Self
    where
        T: Into<http::header::HeaderName>,
    {
        self.headers.insert(
            key.into(),
            http::header::HeaderValue::from_str(val).unwrap(),
        );
        self
    }

    /// Add header to the request
    pub fn add_param<U>(mut self, key: &'static str, val: U) -> Self
    where
        U: Into<String>,
    {
        self.params.insert(key, val.into());
        self
    }

    /// Add header to the request
    /// TODO: Propagate this error.
    pub fn add_param_from_str(mut self, key: &'static str, val: &'static str) -> Self {
        self.params.insert(key, String::from(val));
        self
    }

    /// Add form data to the request
    pub fn add_form<T>(mut self, key: &'static str, val: T) -> Self
    where
        T: Into<String>,
    {
        self.form.insert(key, val.into());
        self
    }

    /// Add form data to the request
    pub fn add_form_from_str<'a, T>(mut self, key: &'static str, val: &'a str) -> Self {
        self.form.insert(key, String::from(val));
        self
    }

    /// TODO: Propagate this error.
    /// Sets query using `serde_urlencoded`
    pub fn finish(mut self) -> Self {
        let mut uri_parts = self.url.into_parts();
        let path = uri_parts.path_and_query;
        let mut buffer = BytesWriter::with_smol_capacity();

        let query = serde_urlencoded::to_string(&self.params).expect("To url-encode");
        let _ = match path {
            Some(path) => write!(buffer, "{}?{}", path.path(), query),
            None => write!(buffer, "?{}", query),
        };

        uri_parts.path_and_query = Some(
            http::uri::PathAndQuery::from_shared(buffer.into_inner().freeze())
                .expect("To create path and query"),
        );

        self.url = match http::Uri::from_parts(uri_parts) {
            Ok(uri) => uri,
            Err(error) => panic!("Unable to set query for URI: {}", error),
        };

        self
    }
}
