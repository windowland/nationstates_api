mod client;
mod error;
pub mod query;
pub mod request;
pub use client::{NsClient, NsRequest};
pub use error::NsError;
use lazy_static::lazy_static;
lazy_static! {
    static ref USER_AGENT: Option<String> = std::env::var("NS_USER_AGENT").ok();
}
///Returns the default user agent used by this script, if one exists.
/// It is set via the `NS_USER_AGENT` environment variable.
pub fn get_user_agent() -> Option<&'static str> {
    USER_AGENT.as_deref()
}
///Url of the nationstates api.
pub const NS_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi";
///The version of the api this crate supports.
pub const API_VERSION: usize = 11;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
