use pixieve_rs::pixiv::request_builder::PixivRequestBuilder;

#[test]
fn test_into_iterator() {
    let slice: &[usize] = &[0, 1, 2];
    let vec = slice.to_owned();
    let iter = vec.clone().into_iter().chain(Some(3));

    PixivRequestBuilder::favorite_works_remove(slice);
    PixivRequestBuilder::favorite_works_remove(vec.clone());
    PixivRequestBuilder::favorite_works_remove(iter.clone());

    PixivRequestBuilder::following_remove(slice);
    PixivRequestBuilder::following_remove(vec);
    PixivRequestBuilder::following_remove(iter);
}
