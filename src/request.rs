//!This module contains traits to abstract away the difference between various
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
    ///Response type returned by the request
    type Response: Response;
    ///Future returned by send.
    type Output: Future<Output = Result<Self::Response, Self::Error>> + Unpin + Send;
    ///The request builder type associated with this
    type Builder: RequestBuilder;
    ///Sends the request, returning a future that resolves to the output.
    /// The server returning a 4XX or a 5XX response code should not be considered an error.
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
    /// should be reported when the request is built.
    fn add_queries<S: Serialize>(self, query: &S) -> Self;
    /// Builds the request, returning an error if something went wrong when building.
    fn build(self) -> Result<Self::Built, Self::BuildError>;
}

///Represents an http reponse from a server.
pub trait Response {
    ///Future that will resolve to the request body.
    type Future: Future<Output = Result<Bytes, Self::Error>> + Unpin + Send;
    /// Error returned when body can't be read.
    type Error;
    ///gets the value of the header represented by name, if one exists.
    fn get_header(&self, name: &str) -> Option<&[u8]>;
    ///gets the status code of the response.
    fn status_code(&self) -> u16;

    ///gets the body of response, consuming self.
    fn body(self) -> Self::Future;
}
#[cfg(feature = "reqwest")]
mod reqwest {
    use ::reqwest::Client;
    use ::reqwest::Error;
    use ::reqwest::Request;
    use ::reqwest::RequestBuilder;
    use ::reqwest::Response;
    use bytes::Bytes;
    use serde::Serialize;
    use std::future::Future;
    use std::pin::Pin;
    impl super::Client for Client {
        type Error = Error;
        type Output = Pin<Box<dyn Future<Output = Result<Response, Self::Error>> + Send>>;
        type Builder = RequestBuilder;
        type Response = Response;
        fn send(&self, request: Request) -> Self::Output {
            Box::pin(self.execute(request))
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
    impl super::Response for Response {
        type Error = Error;
        type Future = Pin<Box<dyn Future<Output = Result<Bytes, Error>> + Send>>;
        fn get_header(&self, name: &str) -> Option<&[u8]> {
            self.headers().get(name).map(|header| header.as_bytes())
        }
        fn status_code(&self) -> u16 {
            self.status().as_u16()
        }
        fn body(self) -> Self::Future {
            Box::pin(self.bytes())
        }
    }
}
