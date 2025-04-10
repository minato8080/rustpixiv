use pixieve_rs::enums::Visibility;
use pixieve_rs::pixiv::arg::illustration_ranking_request_arg::IllustrationRankingRequestArg;
use pixieve_rs::pixiv::arg::illustration_search_request_arg::IllustrationSearchRequestArg;
use pixieve_rs::pixiv::arg::recommended_illustration_request_arg::RecommendedIllustrationRequestArg;
use pixieve_rs::pixiv::client::PixivClient;

use pixieve_rs::pixiv::request_builder::PixivRequestBuilder;

use pixieve_rs::pixiv::result::illustration_bookmark_info_proxy::IllustBookmarkInfoProxy;
use pixieve_rs::pixiv::result::illustration_comment::IllustrationComment;
use pixieve_rs::pixiv::result::illustration_proxy::IllustrationProxy;
use pixieve_rs::pixiv::result::illustration_ranking::IllustrationRanking;
use pixieve_rs::pixiv::result::illustration_search_proxy::IllustrationSearchProxy;
use pixieve_rs::pixiv::result::recommended_illustration::RecommendedIllustration;
use pixieve_rs::pixiv::result::related_illustration_search_proxy::RelatedIllustrationSearchProxy;
use pixieve_rs::pixiv::result::trending_illustrations::TrendingIllustrations;

const ILLUST_ID_TEST: usize = 75523989;

#[test]
fn test_fetching_illustration() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_illustration(ILLUST_ID_TEST);

    let illustration = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationProxy>()
        .expect("Failed to parse as json.")
        .into_inner();

    pixiv.download_illustration(&illustration, &std::env::current_dir().unwrap());
}

#[test]
fn test_search_illustration() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let arg = IllustrationSearchRequestArg::new("Pretty Cure");

    let request = PixivRequestBuilder::request_illustration_search(arg);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationSearchProxy>()
        .expect("Failed to parse as json.");
}

// TODO: JSON Parse error
#[test]
fn test_fetch_illustration_comments() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_illustration_comments(ILLUST_ID_TEST, 0, false);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationComment>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_related_illustrations() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_related_illustration(ILLUST_ID_TEST, 0);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<RelatedIllustrationSearchProxy>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_illustrations_by_followed_artists() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_illustration_following(true);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<RelatedIllustrationSearchProxy>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_recommended_illustrations() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let args = RecommendedIllustrationRequestArg::default();

    let request = PixivRequestBuilder::request_recommended_illustration(args);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<RecommendedIllustration>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_illustrations_ranking() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let args = IllustrationRankingRequestArg::default();

    let request = PixivRequestBuilder::request_illustrations_ranking(args);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationRanking>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_trending_tags() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_trending_tags();

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<TrendingIllustrations>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_illustration_bookmark_info() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_illustration_bookmark_info(ILLUST_ID_TEST);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustBookmarkInfoProxy>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_adding_bookmark() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_adding_bookmark(ILLUST_ID_TEST, Visibility::Public);

    println!("request:\n{:?}", request);

    let result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<serde_json::Value>()
        .expect("Failed to parse as json.");
    println!("result:\n{}", result);
}

#[test]
fn test_delete_bookmark() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let refresh_token = std::env::var("REFRESH_TOKEN").expect("REFRESH_TOKEN isn't set!");
    *pixiv.refresh_token_mut() = refresh_token;

    pixiv.refresh_auth().expect("Failed to log in.");

    let request = PixivRequestBuilder::request_delete_bookmark(ILLUST_ID_TEST);

    println!("request:\n{:?}", request);

    let result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<serde_json::Value>()
        .expect("Failed to parse as json.");

    println!("result:\n{}", result);
}
