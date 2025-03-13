use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    line_module_attach_api: Box<dyn crate::apis::LineModuleAttachApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            line_module_attach_api: Box::new(crate::apis::LineModuleAttachApiClient::new(rc.clone())),
        }
    }

    pub fn line_module_attach_api(&self) -> &dyn crate::apis::LineModuleAttachApi{
        self.line_module_attach_api.as_ref()
    }

}
