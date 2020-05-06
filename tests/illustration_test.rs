use pixiv::pixiv::client::PixivClient;
use pixiv::pixiv::helper_structs::illustration::illustration_bookmark_info_proxy::IllustBookmarkInfoProxy;
use pixiv::pixiv::helper_structs::illustration::illustration_comment::IllustrationComment;
use pixiv::pixiv::helper_structs::illustration::illustration_proxy::IllustrationProxy;
use pixiv::pixiv::helper_structs::illustration::illustration_ranking::IllustrationRanking;
use pixiv::pixiv::helper_structs::illustration::illustration_search_proxy::IllustrationSearchProxy;
use pixiv::pixiv::helper_structs::illustration::recommended_illustration::RecommendedIllustration;
use pixiv::pixiv::helper_structs::illustration::related_illustration_search_proxy::RelatedIllustrationSearchProxy;
use pixiv::pixiv::helper_structs::illustration::req_illust_ranking_arg::IllustRankingArg;
use pixiv::pixiv::helper_structs::illustration::req_rec_illust_arg::IllustRecArg;
use pixiv::pixiv::helper_structs::illustration::trending_illustrations::TrendingIllustrations;
use pixiv::pixiv::request_builder::PixivRequestBuilder;

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

    let request = PixivRequestBuilder::request_illustration(ILLUST_ID_TEST).build();

    let illustration = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationProxy>()
        .expect("Failed to parse as json.")
        .into_inner();

    pixiv.download_illustration(&illustration, &std::env::current_dir().unwrap());
    println!("{:#?}", illustration);
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

    let request = PixivRequestBuilder::request_illustration_search("Pretty Cure").build();

    let result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationSearchProxy>()
        .expect("Failed to parse as json.");

    println!("search illustrations:{:#?}", result);
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

    let request =
        PixivRequestBuilder::request_illustration_comments(ILLUST_ID_TEST, 0, false).build();

    let result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationComment>()
        .expect("Failed to parse as json.");

    println!("comments:{:#?}", result);
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

    let request = PixivRequestBuilder::request_related_illustration(ILLUST_ID_TEST, 0).build();

    let _result = pixiv
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

    let request = PixivRequestBuilder::request_illustration_following(true).build();

    let _result = pixiv
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

    let request = PixivRequestBuilder::request_recommended_illustration(args).build();

    let _result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<RecommendedIllustration>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_illustration_recommendations() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let args = IllustRankingArg::default();

    let request = PixivRequestBuilder::request_illustration_ranking(args).build();

    let _result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustrationRanking>()
        .expect("Failed to parse as json.");
}

#[test]
fn test_fetch_trending_illustrations() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv
        .login(&username, &password)
        .expect("Failed to log in.");

    let request = PixivRequestBuilder::request_trending_tags().build();

    let _result = pixiv
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

    let request = PixivRequestBuilder::request_illustration_bookmark_info(ILLUST_ID_TEST).build();

    let _result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<IllustBookmarkInfoProxy>()
        .expect("Failed to parse as json.");
}
