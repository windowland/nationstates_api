use crate::NsError;
use std::time::Duration;
use tokio::time::{interval, Interval};
#[derive(Debug)]
pub struct NsClient<C> {
    client: C,
    throttle: Interval,
}

impl<C> NsClient<C> {
    ///Creates a new NsClient, with a default rate of 50 requests per 35 seconds. This is purposefully
    /// slightly less than the api's rate limit, to reduce the odds of something like network latency
    /// causing the request to accidentally exceed the rate limit. If this is undesirable, use [`new_with_throttle`]
    /// instead.
    pub fn new(client: C) -> Self {
        NsClient {
            client,
            throttle: interval(Duration::from_secs_f64(50.0 / 35.0)),
        }
    }
    ///Creates a new client with a specified throttle. Sending more than 50 requests in 30 seconds will
    /// result in a 15 minute ban.
    pub fn new_with_throttle(client: C, throttle: Interval) -> Self {
        NsClient { client, throttle }
    }
}
use crate::request::{Client, RequestBuilder};
impl<C: Client> NsClient<C> {
    ///creates a request builder requesting the specified queries.
    pub fn request_builder<Q>(
        &mut self,
        queries: Q,
    ) -> NsRequest<'_, C, <C as Client>::Builder, Q> {
        let builder = self.client.get(crate::NS_URL);
        NsRequest {
            client: self,
            builder,
            queries,
        }
    }
}
/// A request that will be sent to query data from the api.
#[derive(Debug)]
pub struct NsRequest<'c, C, B, Q> {
    client: &'c mut NsClient<C>,
    builder: B,
    queries: Q,
}
use crate::query::Query;
impl<'c, C: Client<Builder = B>, B: RequestBuilder, Q: Query> NsRequest<'c, C, B, Q> {
    pub async fn send(self) -> Result<NsResponse<C::Response>, NsError<B::BuildError, C::Error>> {
        self.send_with_user_agent(crate::get_user_agent().expect("User agent must be set."))
            .await
    }
    pub async fn send_with_user_agent(
        self,
        user_agent: &str,
    ) -> Result<NsResponse<C::Response>, NsError<B::BuildError, C::Error>> {
        let NsRequest {
            client,
            builder,
            queries,
        } = self;
        let builder = queries
            .add_query(builder)
            .add_header("User-Agent", user_agent);
        client.throttle.tick().await;
        let built = builder.build().map_err(NsError::BuildError)?;
        let response = client
            .client
            .send(built)
            .await
            .map_err(NsError::SendError)?;
        Ok(NsResponse { response: response })
    }
}

pub struct NsResponse<R> {
    response: R,
}
