use regex::Regex;

pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_EMPTY: &str = "";
pub const AUTHORIZATION_HEADER: &str = "Authorization";
pub const BEARER_PREFIX: &str = "BEARER";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid Token";
pub const MESSAGE_ALREADY_LOGGED_IN: &str = "Already logged in";
pub const UNPROTECTED_ROUTES: [&str; 3] = ["/ping", "/auth/register", LOGIN_ROUTE];
pub const LOGIN_ROUTE: &str = "/auth/login";
pub const TRANSFER_CREATE_ROUTE: &str = "/transfer/create";
pub const MESSAGE_UNAUTHORIZED: &str = "Unauthorized to make this request";
pub const MESSAGE_INVALID_PAYLOAD: &str = "Invalid payload";
