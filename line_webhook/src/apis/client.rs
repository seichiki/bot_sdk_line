use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    dummy_api: Box<dyn crate::apis::DummyApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            dummy_api: Box::new(crate::apis::DummyApiClient::new(rc.clone())),
        }
    }

    pub fn dummy_api(&self) -> &dyn crate::apis::DummyApi{
        self.dummy_api.as_ref()
    }

}
