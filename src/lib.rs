pub mod request;
use lazy_static::lazy_static;
lazy_static! {
    static ref USER_AGENT: Option<String> = std::env::var("NS_USER_AGENT").ok();
}
///Returns the default user agent used by this script, if one exists.
/// It is set via the `NS_USER_AGENT` environment variable.
pub fn get_user_agent() -> Option<&'static str> {
    USER_AGENT.as_deref()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
