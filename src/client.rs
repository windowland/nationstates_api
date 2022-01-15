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
    /// causing the request
    pub fn new(client: C) -> Self {
        NsClient {
            client,
            throttle: interval(Duration::from_secs_f64(50.0 / 35.0)),
        }
    }
}
use crate::request::Client;
impl<C: Client> NsClient<C> {
    pub fn request_builder(&mut self) -> NsRequestBuilder<'_, C, <C as Client>::Builder> {
        let builder = self.client.get(crate::NS_URL);
        NsRequestBuilder {
            client: self,
            builder,
        }
    }
}

/// A request that will be sent to query data from the api.
pub struct NsRequestBuilder<'c, C, R> {
    client: &'c mut NsClient<C>,
    builder: R,
}
