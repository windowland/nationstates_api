//!This module contains types and trait to abstract away the difference between various
//! http client apis. By default, the traits are implemented by [reqwest](https://crates.io/crates/reqwest)
//! types, although the traits are fairly straightforward to implement for other http libraries.
//! Users of this library should never need to use the methods of these traits themselves,
//! although they may want to implement them for custom types.

use bytes::Bytes;
use serde::Serialize;
use std::future::Future;
/// This trait is designed for use by http clients.
pub trait Client {
    ///Error returned if the request fails.
    type Error;
    ///Future returned by send.
    type Output: Future<Output = Result<Bytes, Self::Error>> + Unpin + Send;
    type Builder: RequestBuilder;
    ///Sends the request, returning a future that resolves to the output.
    /// The server returning a 4XX or a 5XX response code should be considered an error.
    fn send(&self, request: <Self::Builder as RequestBuilder>::Built) -> Self::Output;
    ///Returns a configurable builder that represents a get request to the url.
    /// This function should not panic, instead errors should be reported when
    /// the request is built.
    fn get(&self, url: &str) -> Self::Builder;
}

///Trait that represents a builder for a request.
pub trait RequestBuilder {
    /// Type of the built request
    type Built;
    /// Error type returned when the build fails.
    type BuildError;
    /// Adds a header value pair to the request. Should not panic, instead errors should
    /// be reported when the request is built.
    fn add_header(self, name: &str, value: &str) -> Self;
    /// Adds serializable values to the query string. Should not panic, instead errors
    /// be reported when the request is built.
    fn add_queries<S: Serialize>(self, query: &S) -> Self;
    /// Builds the request, returning an error if something went wrong when building.
    fn build(self) -> Result<Self::Built, Self::BuildError>;
}

#[cfg(feature = "reqwest")]
mod reqwest {
    use ::reqwest::Client;
    use ::reqwest::Error;
    use ::reqwest::Request;
    use ::reqwest::RequestBuilder;
    use bytes::Bytes;
    use futures::TryFutureExt;
    use serde::Serialize;
    use std::future::Future;
    use std::pin::Pin;
    impl super::Client for Client {
        type Error = Error;
        type Output = Pin<Box<dyn Future<Output = Result<Bytes, Self::Error>> + Send>>;
        type Builder = RequestBuilder;
        fn send(&self, request: Request) -> Self::Output {
            Box::pin(self.execute(request).and_then(|r| r.bytes()))
        }
        fn get(&self, url: &str) -> Self::Builder {
            self.get(url)
        }
    }
    impl super::RequestBuilder for RequestBuilder {
        type Built = Request;
        type BuildError = Error;
        fn add_header(self, name: &str, value: &str) -> Self {
            self.header(name, value)
        }
        fn add_queries<S: Serialize>(self, query: &S) -> Self {
            self.query(query)
        }
        fn build(self) -> Result<Self::Built, Self::BuildError> {
            self.build()
        }
    }
}
