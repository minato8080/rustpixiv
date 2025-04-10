use crate::constants::{
    AUTH_URL, CLIENT_ID, CLIENT_SECRET, HASH_SECRET, USER_AGENT, X_CLIENT_HASH, X_CLIENT_TIME,
};
use crate::errors::AuthError;
use crate::pixiv::helper_structs::illustration::Illustration;
use crate::pixiv::request::PixivRequest;

use http::{header, status::StatusCode};
use md5;
use reqwest::{Client, ClientBuilder, Response};
use serde_json::Value;

/// Used to authenticate to the PixivClient servers and construct PixivClient requests through methods creating `PixivRequestBuilder`.
#[derive(Debug, Clone)]
pub struct PixivClient {
    pub client: Client,
    pub access_token: String,
    pub refresh_token: String,
}

impl PixivClient {
    /// Creates a new PixivClient struct.

    pub fn new() -> Result<PixivClient, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(PixivClient {
            client: client,
            access_token: String::default(),
            refresh_token: String::default(),
        })
    }
    /// This is required to use all the other functions this library provides. Requires a valid username and password.
    pub async fn login<'a, 'b, 'c>(
        &'a mut self,
        username: &'b str,
        password: &'c str,
    ) -> Result<(), AuthError> {
        let mut data = std::collections::HashMap::new();

        data.insert("get_secure_url", "true");
        data.insert("client_id", CLIENT_ID);
        data.insert("client_secret", CLIENT_SECRET);
        data.insert("refresh_token", "");
        data.insert("grant_type", "password");
        data.insert("username", username);
        data.insert("password", password);

        let res = self
            .send_auth_request(&data)
            .await
            .expect("Error occured while requesting token.");

        match res.status() {
            StatusCode::OK | StatusCode::MOVED_PERMANENTLY | StatusCode::FOUND => (),
            s => {
                return Err(AuthError::because(format!(
                    "Login failed. Check your username and password. Response: {:?}",
                    s
                )))
            }
        }

        // TODO: Figure out the correct struct for this
        let mut json_response: Value = res.json().await.unwrap();

        self.access_token = match json_response["response"]["access_token"].take() {
            Value::String(s) => s,
            _ => panic!("Failed to get access token."),
        };
        self.refresh_token = match json_response["response"]["refresh_token"].take() {
            Value::String(s) => s,
            _ => panic!("Failed to get refresh token."),
        };

        Ok(())
    }

    /// Refreshes the authentication. You should use this when your access token is close to expiring.
    pub async fn refresh_auth(&mut self) -> Result<(), AuthError> {
        let refresh_clone = self.refresh_token.clone();
        let mut data = std::collections::HashMap::new();

        data.insert("client_id", CLIENT_ID);
        data.insert("client_secret", CLIENT_SECRET);
        data.insert("get_secure_url", "true");
        data.insert("grant_type", "refresh_token");
        data.insert("refresh_token", refresh_clone.as_str());
        data.insert("include_policy", "true");

        let res = self
            .send_auth_request(&data)
            .await
            .expect("Error occured while requesting token.");

        match res.status() {
            StatusCode::OK | StatusCode::MOVED_PERMANENTLY | StatusCode::FOUND => {
                // success
            }
            s => {
                return Err(AuthError::because(format!(
                    "Login failed. Check your refresh token. Response: {:?}",
                    s
                )))
            }
        }

        let mut json_response: Value = match res.json().await {
            Ok(json) => json,
            Err(e) => panic!("Failed to parse JSON: {}", e),
        };

        self.access_token = match json_response["response"]["access_token"].take() {
            Value::String(s) => s,
            _ => panic!("Failed to get access token."),
        };
        self.refresh_token = match json_response["response"]["refresh_token"].take() {
            Value::String(s) => s,
            _ => panic!("Failed to get refresh token."),
        };
        Ok(())
    }

    /// Get the access token.

    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    /// Get a mutable reference to the access token.

    pub fn access_token_mut(&mut self) -> &mut String {
        &mut self.access_token
    }

    /// Get the refresh token.

    pub fn refresh_token(&self) -> &String {
        &self.refresh_token
    }

    /// Get a mutable reference to the refresh token.

    pub fn refresh_token_mut(&mut self) -> &mut String {
        &mut self.refresh_token
    }

    /// Get current UTC time as a `String`.
    fn get_current_time(&self) -> String {
        chrono::offset::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    }

    /// Get the client's MD5 hash.
    fn get_client_hash(&self, time: &String) -> String {
        format!("{:x}", md5::compute(format!("{}{}", time, HASH_SECRET)))
    }

    /// Private helper method
    async fn send_auth_request(
        &self,
        data: &std::collections::HashMap<&str, &str>,
    ) -> Result<Response, reqwest::Error> {
        let client_time = self.get_current_time();
        let client_hash = self.get_client_hash(&client_time);

        self.client
            .post(AUTH_URL)
            .header(X_CLIENT_TIME, client_time)
            .header(X_CLIENT_HASH, client_hash)
            .header("accept-language", "en_US")
            .header("host", "oauth.secure.pixiv.net")
            .header("app-os", "ios")
            .header("app-os-version", "14.6")
            .header("user-agent", "PixivIOSApp/7.13.3 (iOS 14.6; iPhone13,2)")
            .header("content-type", "application/x-www-form-urlencoded")
            .form(&data)
            .send()
            .await
    }

    /// Executes a given `PixivRequest`.
    /// TODO: Add another function that can execute without authentication (is there even anything like this?)
    pub async fn execute_with_auth(
        &self,
        request: PixivRequest,
    ) -> Result<Response, reqwest::Error> {
        let uri = format!("{}", request.url);
        let url = reqwest::Url::parse(&uri).unwrap();
        self.client
            .request(request.method, url)
            .headers(request.headers)
            .form(&request.form)
            .bearer_auth(self.access_token.clone())
            .send()
            .await
    }

    /// Download a given illustration to path
    pub async fn download_illustration<'a, 'b, 'c>(
        &'a self,
        illustration: &'c Illustration,
        path: &'b std::path::Path,
    ) {
        illustration.download(&self.client, &path).await;
    }
}
