use pixieve_rs::pixiv::client::PixivClient;
use pixieve_rs::pixiv::request_builder::PixivRequestBuilder;

use serde_json::Value;

#[test]
fn test_login() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");
}

#[test]
fn test_refresh_auth() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    pixiv
        .refresh_auth()
        .expect("Failed to refresh access token");
}

#[test]
fn test_request_bad_words() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::request_bad_words();
    let bad_words: Value = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", bad_words);
}

#[test]
fn test_work() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::work(66024340);
    let work: Value = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", work);
}

#[test]
fn test_user() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::user(6996493);
    let following_works: Value = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", following_works);
}

#[test]
fn test_following_works() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::following_works(&["large"], false).finish();
    let following_works: Value = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json()
        .expect("Failed to parse as json.");

    println!("{}", following_works);
}

#[test]
#[should_panic]
fn test_login_fail() {
    let mut pixiv: PixivClient = PixivClient::new().unwrap();
    pixiv.login("", "").expect("Failed to log in.");
}
