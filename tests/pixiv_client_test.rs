use pixieve_rs::pixiv::client::PixivClient;
use pixieve_rs::pixiv::request_builder::PixivRequestBuilder;

use serde_json::Value;

// This way is not work
#[tokio::test]
async fn test_login() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .await
        .expect("Failed to log in.");
}

#[tokio::test]
async fn test_refresh_auth() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv
        .refresh_auth()
        .await
        .expect("Failed to log in.");

    pixiv
        .refresh_auth()
        .await
        .expect("Failed to refresh access token");
}

#[tokio::test]
async fn test_request_bad_words() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv
        .refresh_auth()
        .await
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::request_bad_words();
    let bad_words: Value = pixiv
        .execute_with_auth(request)
        .await
        .expect("Request failed.")
        .json()
        .await
        .expect("Failed to parse as json.");

    println!("{}", bad_words);
}

#[tokio::test]
async fn test_work() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv
        .refresh_auth()
        .await
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::work(66024340);
    let work: Value = pixiv
        .execute_with_auth(request)
        .await
        .expect("Request failed.")
        .json()
        .await
        .expect("Failed to parse as json.");

    println!("{}", work);
}

#[tokio::test]
async fn test_user() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv
        .refresh_auth()
        .await
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::user(6996493);
    let following_works: Value = pixiv
        .execute_with_auth(request)
        .await
        .expect("Request failed.")
        .json()
        .await
        .expect("Failed to parse as json.");

    println!("{}", following_works);
}

#[tokio::test]
async fn test_following_works() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv
        .refresh_auth()
        .await
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::following_works(&["large"], false).finish();
    let following_works: Value = pixiv
        .execute_with_auth(request)
        .await
        .expect("Request failed.")
        .json()
        .await
        .expect("Failed to parse as json.");

    println!("{}", following_works);
}

#[tokio::test]
#[should_panic]
async fn test_login_fail() {
    let mut pixiv: PixivClient = PixivClient::new().unwrap();
    pixiv.login("", "").await.expect("Failed to log in.");
}
