use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    insight_api: Box<dyn crate::apis::InsightApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            insight_api: Box::new(crate::apis::InsightApiClient::new(rc.clone())),
        }
    }

    pub fn insight_api(&self) -> &dyn crate::apis::InsightApi{
        self.insight_api.as_ref()
    }

}
