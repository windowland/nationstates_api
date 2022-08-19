use std::{sync::Arc, time::Duration};

use crate::USER_AGENT;
use reqwest::Client;
use tokio::sync::Semaphore;
///The main struct for sending requests to the nationstates api. Note that
/// clones are shallow and cheap.
#[derive(Debug, Clone)]
pub struct NsClient {
    client: Client,
    throttle: Duration,
    slot: Arc<Semaphore>,
}

impl NsClient {
    ///Returns a client with the default setings.
    pub fn new() -> Self {
        NsClient {
            client: Client::builder()
                .user_agent(USER_AGENT.as_deref().expect("User-Agent not set"))
                .https_only(true)
                .build()
                .unwrap(),
            throttle: Duration::from_secs(35),
            slot: Arc::new(Semaphore::new(15)),
        }
    }
}
#[derive(Debug, Clone)]
pub struct NsClientBuilder {
    user_agent: Option<String>,
    throttle: Option<Duration>,
}

impl NsClientBuilder {
    pub fn new() -> Self {
        NsClientBuilder {
            user_agent: None,
            throttle: None,
        }
    }
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    pub fn throttle(mut self, throttle: Duration) -> Self {
        self.throttle = Some(throttle);
        self
    }
    pub fn build(self) -> NsClient {
        NsClient {
            client: Client::builder()
                .user_agent(self.user_agent.unwrap_or_else(|| {
                    USER_AGENT
                        .as_deref()
                        .expect("User-Agent not set")
                        .to_owned()
                }))
                .https_only(true)
                .build()
                .unwrap(),
            throttle: self.throttle.unwrap_or_else(|| Duration::from_secs(35)),
            slot: Arc::new(Semaphore::new(15)),
        }
    }
}
