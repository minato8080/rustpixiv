use super::{AuthError, PixivRequest};
use http::{header, status::StatusCode};
use md5;
use reqwest::{Client, ClientBuilder, Response};
use serde_json::Value;

// This is taken from the Android app, don't worry about it. It's not really "compromisable", to some degree.
const AUTH_URL: &str = "https://oauth.secure.pixiv.net/auth/token";
const CLIENT_ID: &str = "KzEZED7aC0vird8jWyHM38mXjNTY";
const CLIENT_SECRET: &str = "W9JZoJe00qPvJsiyCGT3CCtC6ZUtdpKpzMbNlUGP";
const HASH_SECRET: &str = "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c";
const USER_AGENT: &str = "PixivAndroidApp/5.0.64 (Android 6.0)";
const X_CLIENT_TIME: &str = "X-Client-Time";
const X_CLIENT_HASH: &str = "X-Client-Hash";

/// Used to authenticate to the Pixiv servers and construct Pixiv requests through methods creating `PixivRequestBuilder`.
#[derive(Debug, Clone)]
pub struct Pixiv {
    client: Client,
    access_token: String,
    refresh_token: String,
}

impl Pixiv {
    /// Creates a new Pixiv struct.
    #[inline]
    pub fn new() -> Result<Pixiv, reqwest::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(USER_AGENT),
        );

        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(Pixiv {
            client: client,
            access_token: String::default(),
            refresh_token: String::default(),
        })
    }
    /// This is required to use all the other functions this library provides. Requires a valid username and password.
    pub fn login(&mut self, username: &str, password: &str) -> Result<(), AuthError> {
        let mut data = std::collections::HashMap::new();

        data.insert("get_secure_url", "1");
        data.insert("client_id", CLIENT_ID);
        data.insert("client_secret", CLIENT_SECRET);

        data.insert("grant_type", "password");
        data.insert("username", username);
        data.insert("password", password);

        println!("data:{:#?}", data);
        let mut res = self
            .send_auth_request(&data)
            .expect("Error occured while requesting token.");

        match res.status() {
            StatusCode::OK | StatusCode::MOVED_PERMANENTLY | StatusCode::FOUND => (),
            s => {
                return Err(AuthError {
                    reason: format!(
                        "Login failed. Check your username and password. Response: {:?}",
                        s
                    ),
                })
            }
        }

        let mut json_response: Value = res.json().unwrap();

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
    pub fn refresh_auth(&mut self) -> Result<(), AuthError> {
        let refresh_clone = self.refresh_token.clone();
        let mut data = std::collections::HashMap::new();

        data.insert("client_id", CLIENT_ID);
        data.insert("client_secret", CLIENT_SECRET);
        data.insert("get_secure_url", "1");
        data.insert("grant_type", "refresh_token");
        data.insert("refresh_token", refresh_clone.as_str());

        let mut res = self
            .send_auth_request(&data)
            .expect("Error occured while requesting token.");

        match res.status() {
            StatusCode::OK | StatusCode::MOVED_PERMANENTLY | StatusCode::FOUND => {
                // success
            }
            s => {
                return Err(AuthError {
                    reason: format!("Login failed. Check your refresh token. Response: {:?}", s),
                })
            }
        }

        let mut json_response: Value = res.json().unwrap();

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
    #[inline]
    pub fn access_token(&self) -> &String {
        &self.access_token
    }

    /// Get a mutable reference to the access token.
    #[inline]
    pub fn access_token_mut(&mut self) -> &mut String {
        &mut self.access_token
    }

    /// Get the refresh token.
    #[inline]
    pub fn refresh_token(&self) -> &String {
        &self.refresh_token
    }

    /// Get a mutable reference to the refresh token.
    #[inline]
    pub fn refresh_token_mut(&mut self) -> &mut String {
        &mut self.refresh_token
    }

    /// Get current UTC time as a `String`.
    fn get_current_time(&self) -> String {
        chrono::offset::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, false)
    }

    /// Get the client's MD5 hash.
    fn get_client_hash(&self, time: &String) -> String {
        format!("{:x}", md5::compute(format!("{}{}", time, HASH_SECRET)))
    }

    /// Private helper method
    fn send_auth_request(
        &self,
        data: &std::collections::HashMap<&str, &str>,
    ) -> Result<Response, reqwest::Error> {
        let client_time = self.get_current_time();
        let client_hash = self.get_client_hash(&client_time);

        println!(
            "client-time:{}\nclient-hash:{}\ndata:{:#?}",
            client_time, client_hash, data
        );

        let req = self
            .client
            .post(AUTH_URL)
            .header(X_CLIENT_TIME, client_time)
            .header(X_CLIENT_HASH, client_hash)
            .form(&data);

        println!("req:{:#?}", req);
        req.send()
    }

    /// Executes a given `PixivRequest`.
    pub fn execute(&self, request: PixivRequest) -> Result<Response, reqwest::Error> {
        let uri = format!("{}", request.url);
        let url = reqwest::Url::parse(&uri).unwrap();
        self.client
            .request(request.method, url)
            .headers(request.headers)
            .bearer_auth(self.access_token.clone())
            .send()
    }
}

#[cfg(test)]
mod tests {
    use super::Pixiv;
    use serde_json::Value;

    use super::super::*;

    #[test]
    fn test_login() {
        dotenv::dotenv().ok();

        let mut pixiv: Pixiv = Pixiv::new().unwrap();

        let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
        let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

        println!("username:{}\npassword:{}", username, password);

        pixiv.login(&username, &password).expect("Failed to log in");
    }

    #[test]
    fn test_refresh_auth() {
        dotenv::dotenv().ok();

        let mut pixiv: Pixiv = Pixiv::new().unwrap();

        let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
        let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

        println!("username:{}\npassword:{}", username, password);

        pixiv
            .login(&username, &password)
            .expect("Failed to log in.");

        pixiv
            .refresh_auth()
            .expect("Failed to refresh access token");
    }

    #[test]
    fn test_bad_words() {
        dotenv::dotenv().ok();

        let mut pixiv: Pixiv = Pixiv::new().unwrap();

        let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
        let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

        println!("username:{}\npassword:{}", username, password);

        pixiv
            .login(&username, &password)
            .expect("Failed to log in.");

        let request = PixivRequestBuilder::bad_words().build();
        let bad_words: Value = pixiv
            .execute(request)
            .expect("Request failed.")
            .json()
            .expect("Failed to parse as json.");

        println!("{}", bad_words);
    }

    #[test]
    fn test_work() {
        dotenv::dotenv().ok();

        let mut pixiv: Pixiv = Pixiv::new().unwrap();

        let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
        let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

        println!("username:{}\npassword:{}", username, password);

        pixiv
            .login(&username, &password)
            .expect("Failed to log in.");

        let request = PixivRequestBuilder::work(66024340).build();
        let work: Value = pixiv
            .execute(request)
            .expect("Request failed.")
            .json()
            .expect("Failed to parse as json.");

        println!("{}", work);
    }

    #[test]
    fn test_user() {
        dotenv::dotenv().ok();

        let mut pixiv: Pixiv = Pixiv::new().unwrap();

        let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
        let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

        println!("username:{}\npassword:{}", username, password);

        pixiv
            .login(&username, &password)
            .expect("Failed to log in.");

        let request = PixivRequestBuilder::user(6996493).build();
        let following_works: Value = pixiv
            .execute(request)
            .expect("Request failed.")
            .json()
            .expect("Failed to parse as json.");

        println!("{}", following_works);
    }

    #[test]
    fn test_following_works() {
        dotenv::dotenv().ok();

        let mut pixiv: Pixiv = Pixiv::new().unwrap();

        let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
        let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

        println!("username:{}\npassword:{}", username, password);

        pixiv.login(&username, &password).expect("Failed to log in");

        let request = PixivRequestBuilder::following_works()
            .image_sizes(&["large"])
            .include_sanity_level(false)
            .build();
        let following_works: Value = pixiv
            .execute(request)
            .expect("Request failed.")
            .json()
            .expect("Failed to parse as json.");

        println!("{}", following_works);
    }

    #[test]
    #[should_panic]
    fn test_login_fail() {
        let mut pixiv: Pixiv = Pixiv::new().unwrap();

        pixiv.login("", "").expect("Failed to log in.");
    }
}
