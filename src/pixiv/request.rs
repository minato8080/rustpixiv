use crate::utils::BytesWriter;

use http::{uri::Uri, HeaderMap, Method};

use std::io::Write;

/// Pixiv request. You can create this using `PixivRequestBuilder::build`. This is for if you wish to inspect the request before sending.
#[derive(Debug, Clone)]
pub struct PixivRequest {
    pub method: Method,
    pub url: Uri,
    pub headers: HeaderMap,
}

impl PixivRequest {
    /// Create a new `PixivRequest`.
    /// A `PixivRequest` is returned when calling `build()` on `PixivRequestBuilder`, so it is recommended you use that instead.

    pub fn new(method: Method, url: Uri, headers: HeaderMap) -> PixivRequest {
        PixivRequest {
            method,
            url,
            headers,
        }
    }
    /// Get the method.

    pub fn method(&self) -> &Method {
        &self.method
    }
    /// Get a mutable reference to the method.

    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.method
    }
    /// Get the url.

    pub fn url(&self) -> &Uri {
        &self.url
    }
    /// Get a mutable reference to the url.

    pub fn url_mut(&mut self) -> &mut Uri {
        &mut self.url
    }
    /// Get the headers.

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    /// Get a mutable reference to the headers.

    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    ///Sets query using `serde_urlencoded`
    pub fn set_query_params<Q: serde::Serialize>(mut self, params: &Q) -> Self {
        let mut uri_parts = self.url.into_parts();
        let path = uri_parts.path_and_query;

        let mut buffer = BytesWriter::with_smol_capacity();
        let query = serde_urlencoded::to_string(params).expect("To url-encode");

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
