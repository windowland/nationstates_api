pub mod request;
use lazy_static::lazy_static;
lazy_static! {
    pub static ref USER_AGENT: Option<String> = std::env::var("NS_USER_AGENT").ok();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
