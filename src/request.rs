//!This module contains types and trait to abstract away the difference between various
//! http client apis. By default, the traits are implemented by [reqwest](https://crates.io/crates/reqwest)
//! types, although the traits are fairly straightforward to implement for most types.
//! Users of this library should never need to use the methods of these traits themselves,
//! although they may want to implement them for custom types.

/// This trait is designed for use by http clients.
pub trait Client {
  /// The builder type.
  type Builder: RequestBuilder;
  ///Returns a configurable builder that represents a get request to the url.
  /// This function should not panic, instead errors should be reported when
  /// the request is built.
  fn get(&self, url: &str) -> Self::Builder;
}

///Trait that represents a builder for a request.
pub trait RequestBuilder {
  /// Type of the built request
  type Built: Request;
  /// Error type returned when the build fails.
  type BuildError;
  /// Adds a header value pair to the request. Should not panic, instead errors should
  /// be reported when the request is built.
  fn add_header(&mut self, name: &str, value: &str);
  /// Adds serializable values to the query string. Should not panic, instead errors
  /// be reported when the request is built.
  fn add_queries<S: Serialize>(&mut self, query: S);
  /// Builds the request, returning an error if something went wrong when building.
  fn build(self) -> Result<Self::Built, Self::BuildError>;
}
use bytes::Bytes;
use serde::Serialize;
use std::future::Future;

///Trait respresenting a sendable http request.
pub trait Request {
  ///Error returned if the request fails.
  type Error;
  ///Future returned by send.
  type Output: Future<Output = Result<Bytes, Self::Error>>;
  ///Sends the request, returning a future that resolves to the output.
  /// The server returning a 4XX or a 5XX response code should be considered an error.
  fn send(self) -> Self::Output;
}
