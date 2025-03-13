use std::sync::Arc;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use super::configuration::Configuration;

pub struct APIClient {
    manage_audience_api: Box<dyn crate::apis::ManageAudienceApi>,
    manage_audience_blob_api: Box<dyn crate::apis::ManageAudienceBlobApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
        where C: Clone + std::marker::Send + Sync + 'static {
        let rc = Arc::new(configuration);

        APIClient {
            manage_audience_api: Box::new(crate::apis::ManageAudienceApiClient::new(rc.clone())),
            manage_audience_blob_api: Box::new(crate::apis::ManageAudienceBlobApiClient::new(rc.clone())),
        }
    }

    pub fn manage_audience_api(&self) -> &dyn crate::apis::ManageAudienceApi{
        self.manage_audience_api.as_ref()
    }

    pub fn manage_audience_blob_api(&self) -> &dyn crate::apis::ManageAudienceBlobApi{
        self.manage_audience_blob_api.as_ref()
    }

}
