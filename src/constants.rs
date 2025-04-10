pub const BASE_URL: &'static str = "https://app-api.pixiv.net";
// This is taken from the Android app, don't worry about it. It's not really "compromisable", to some degree.
pub const AUTH_URL: &'static str = "https://oauth.secure.pixiv.net/auth/token";
pub const CLIENT_ID: &'static str = "MOBrBDS8blbauoSck0ZfDbtuzpyT";
pub const CLIENT_SECRET: &'static str = "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj";
pub const HASH_SECRET: &'static str =
    "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c";
pub const USER_AGENT: &'static str = "PixivAndroidApp/5.0.234 (Android 11; Pixel 5)";
pub const X_CLIENT_TIME: &'static str = "X-Client-Time";
pub const X_CLIENT_HASH: &'static str = "X-Client-Hash";
pub const FOR_IOS: &'static str = "for_ios";

// Header Keys
pub const ILLUST_ID: &'static str = "illust_id";
pub const RESTRICT: &'static str = "restrict";
pub const TAGS: &'static str = "tags[]";
pub const OFFSET: &'static str = "offset";
pub const USER_ID: &'static str = "user_id";

// Header Values
pub const NONE: &'static str = "none";
