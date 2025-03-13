use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    messaging_api_api: Box<dyn crate::apis::MessagingApiApi>,
    messaging_api_blob_api: Box<dyn crate::apis::MessagingApiBlobApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            messaging_api_api: Box::new(crate::apis::MessagingApiApiClient::new(rc.clone())),
            messaging_api_blob_api: Box::new(crate::apis::MessagingApiBlobApiClient::new(rc.clone())),
        }
    }

    pub fn messaging_api_api(&self) -> &dyn crate::apis::MessagingApiApi{
        self.messaging_api_api.as_ref()
    }

    pub fn messaging_api_blob_api(&self) -> &dyn crate::apis::MessagingApiBlobApi{
        self.messaging_api_blob_api.as_ref()
    }

}
