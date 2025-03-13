use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    channel_access_token_api: Box<dyn crate::apis::ChannelAccessTokenApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            channel_access_token_api: Box::new(crate::apis::ChannelAccessTokenApiClient::new(rc.clone())),
        }
    }

    pub fn channel_access_token_api(&self) -> &dyn crate::apis::ChannelAccessTokenApi{
        self.channel_access_token_api.as_ref()
    }

}
