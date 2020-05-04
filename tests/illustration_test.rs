use pixiv::pixiv::client::Pixiv;
use pixiv::pixiv::helper_structs::illustration::illustration_proxy::IllustrationProxy;
use pixiv::pixiv::helper_structs::illustration::illustration_search_proxy::IllustrationSearchProxy;
use pixiv::pixiv::request_builder::PixivRequestBuilder;
use serde_json::Value;

#[test]
fn test_fetching_illustration() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::illustration(75523989).build();

    let illustration = pixiv
        .execute(request)
        .expect("Request failed.")
        .json::<IllustrationProxy>()
        .expect("Failed to parse as json.")
        .into_inner();

    pixiv.download_illustration(&std::env::current_dir().unwrap(), &illustration);
    println!("{:#?}", illustration);
}

#[test]
fn test_search_illustration() {
    dotenv::dotenv().ok();

    let mut pixiv: Pixiv = Pixiv::new().unwrap();

    let username = std::env::var("PIXIV_ID").expect("PIXIV_ID isn't set!");
    let password = std::env::var("PIXIV_PW").expect("PIXIV_PW isn't set!");

    pixiv.login(&username, &password).expect("Failed to log in");

    let request = PixivRequestBuilder::search_illustration("Pretty Cure").build();

    let illustration = pixiv
        .execute(request)
        .expect("Request failed.")
        .json::<IllustrationSearchProxy>()
        .expect("Failed to parse as json.");

    println!("search result:{:#?}", illustration);
}
