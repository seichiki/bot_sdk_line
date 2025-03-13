use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    line_module_api: Box<dyn crate::apis::LineModuleApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            line_module_api: Box::new(crate::apis::LineModuleApiClient::new(rc.clone())),
        }
    }

    pub fn line_module_api(&self) -> &dyn crate::apis::LineModuleApi{
        self.line_module_api.as_ref()
    }

}
