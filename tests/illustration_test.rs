use pixieve_rs::pixiv::client::PixivClient;
use pixieve_rs::pixiv::helper_structs::illustration::illustration_bookmark_info_proxy::IllustBookmarkInfoProxy;
use pixieve_rs::pixiv::helper_structs::illustration::illustration_comment::IllustrationComment;
use pixieve_rs::pixiv::helper_structs::illustration::illustration_proxy::IllustrationProxy;
use pixieve_rs::pixiv::helper_structs::illustration::illustration_ranking::IllustrationRanking;
use pixieve_rs::pixiv::helper_structs::illustration::illustration_search_proxy::IllustrationSearchProxy;
use pixieve_rs::pixiv::helper_structs::illustration::recommended_illustration::RecommendedIllustration;
use pixieve_rs::pixiv::helper_structs::illustration::related_illustration_search_proxy::RelatedIllustrationSearchProxy;
use pixieve_rs::pixiv::helper_structs::illustration::req_illust_ranking_arg::IllustRankingArg;
use pixieve_rs::pixiv::helper_structs::illustration::req_rec_illust_arg::IllustRecArg;
use pixieve_rs::pixiv::helper_structs::illustration::trending_illustrations::TrendingIllustrations;
use pixieve_rs::pixiv::request_builder::PixivRequestBuilder;

const ILLUST_ID_TEST: usize = 75523989;

#[test]
fn test_fetching_illustration() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::request_illustration_search("Pretty Cure");

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationSearchProxy>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_illustration_comments() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let args = IllustRecArg::default();

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let args = IllustRankingArg::default();

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

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

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::request_adding_bookmark(ILLUST_ID_TEST);

    pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustBookmarkInfoProxy>()
        .expect("Failed to parse as json.");
}

