use pixiv::pixiv::client::PixivClient;
use pixiv::pixiv::helper_structs::illustration::illustration_comment::IllustrationComment;
use pixiv::pixiv::helper_structs::illustration::illustration_proxy::IllustrationProxy;
use pixiv::pixiv::helper_structs::illustration::illustration_search_proxy::IllustrationSearchProxy;
use pixiv::pixiv::helper_structs::illustration::related_illustration_search_proxy::RelatedIllustrationSearchProxy;
use pixiv::pixiv::helper_structs::illustration::req_rec_illust_builder::ReqRecIllustArgBuilder;
use pixiv::pixiv::request_builder::PixivRequestBuilder;

#[test]
fn test_fetching_illustration() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::request_illustration(75523989).build();

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

    pixiv.login(&username, &password).expect("Failed to log in");

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

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::request_illustration_comments(75523989, 0, false).build();

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

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::request_related_illustration(75523989, 0).build();

    let result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<RelatedIllustrationSearchProxy>()
        .expect("Failed to parse as json.");

    println!("result:{:#?}", result);
}

#[test]
fn test_fetch_illustrations_by_followed_artists() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::request_illustration_following(true).build();

    let result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<RelatedIllustrationSearchProxy>()
        .expect("Failed to parse as json.");

    println!("result:{:#?}", result);
}

#[test]
fn test_fetch_recommended_illustrations() {
    dotenv::dotenv().ok();

    let mut pixiv: PixivClient = PixivClient::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");

    let args = ReqRecIllustArgBuilder::default();

    let request = PixivRequestBuilder::request_recommended_illustration(args).build();

    let result = pixiv
        .execute_with_auth(request)
        .expect("Request failed.")
        .json::<serde_json::Value>()
        .expect("Failed to parse as json.");

    println!("result:{:#?}", result);
}
