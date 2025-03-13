use std::sync::Arc;

use hyper_util::{client::legacy::Client, rt::TokioExecutor};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_tls::HttpsConnector;

use line_channel_access_token::apis::{
    configuration::Configuration as ChannelAccessTokenApiConfiguration, ChannelAccessTokenApiClient,
};
use line_insight::apis::{configuration::Configuration as InsightConfiguration, InsightApiClient};
use line_liff::apis::{configuration::Configuration as LiffConfiguration, LiffApiClient};
use line_manage_audience::apis::{
    configuration::Configuration as ManageAudienceConfiguration, ManageAudienceApiClient,
};
use line_messaging_api::apis::{
    configuration::Configuration as MessagingApiConfiguration, MessagingApiApiClient,
};
use line_module::apis::{
    configuration::Configuration as LineModuleConfiguration, LineModuleApiClient,
};
use line_module_attach::apis::{
    configuration::Configuration as LineModuleAttachConfiguration, LineModuleAttachApiClient,
};
use line_shop::apis::{configuration::Configuration as ShopConfiguration, ShopApiClient};
use line_webhook::apis::{
    configuration::Configuration as WebhookConfiguration, DummyApiClient as WebhookDummyApiClient,
};

type Connector= HttpsConnector<HttpConnector>;


pub struct LINE {
    pub channel_access_token_api_client: ChannelAccessTokenApiClient<Connector>,
    pub insight_api_client: InsightApiClient<Connector>,
    pub liff_api_client: LiffApiClient<Connector>,
    pub manage_audience_api_client: ManageAudienceApiClient<Connector>,
    pub messaging_api_client: MessagingApiApiClient<Connector>,
    pub module_api_client: LineModuleApiClient<Connector>,
    pub module_attach_api_client: LineModuleAttachApiClient<Connector>,
    pub shop_api_client: ShopApiClient<Connector>,
    pub webhook_dummy_api_client: WebhookDummyApiClient<Connector>,
}

impl LINE {
    pub fn new(token: String) -> LINE {
        let client = LINE::create_hyper_client();

        // channel_access_token_api
        let mut channel_access_token_api_conf =
            ChannelAccessTokenApiConfiguration::<Connector>::new(client.clone());
        channel_access_token_api_conf.oauth_access_token = Some(token.to_owned());
        let channel_access_token_api_client =
            ChannelAccessTokenApiClient::new(Arc::new(channel_access_token_api_conf));

        // insight
        let mut insight_conf = InsightConfiguration::<Connector>::new(client.clone());
        insight_conf.oauth_access_token = Some(token.to_owned());
        let insight_api_client = InsightApiClient::new(Arc::new(insight_conf));

        // liff
        let mut liff_conf = LiffConfiguration::<Connector>::new(client.clone());
        liff_conf.oauth_access_token = Some(token.to_owned());
        let liff_api_client = LiffApiClient::new(Arc::new(liff_conf));

        // manage_audience
        let mut manage_audience_conf = ManageAudienceConfiguration::<Connector>::new(client.clone());
        manage_audience_conf.oauth_access_token = Some(token.to_owned());
        let manage_audience_api_client =
            ManageAudienceApiClient::new(Arc::new(manage_audience_conf));

        // messaging_api
        let mut messaging_api_conf = MessagingApiConfiguration::<Connector>::new(client.clone());
        messaging_api_conf.oauth_access_token = Some(token.to_owned());
        let messaging_api_client = MessagingApiApiClient::new(Arc::new(messaging_api_conf));

        // module
        let mut module_conf = LineModuleConfiguration::<Connector>::new(client.clone());
        module_conf.oauth_access_token = Some(token.to_owned());
        let module_api_client = LineModuleApiClient::new(Arc::new(module_conf));

        // module_attach
        let mut module_attach_conf = LineModuleAttachConfiguration::<Connector>::new(client.clone());
        module_attach_conf.oauth_access_token = Some(token.to_owned());
        let module_attach_api_client = LineModuleAttachApiClient::new(Arc::new(module_attach_conf));

        // shop
        let mut shop_conf = ShopConfiguration::<Connector>::new(client.clone());
        shop_conf.oauth_access_token = Some(token.to_owned());
        let shop_api_client = ShopApiClient::new(Arc::new(shop_conf));

        // webhook
        let mut webhook_conf = WebhookConfiguration::<Connector>::new(client.clone());
        webhook_conf.oauth_access_token = Some(token.to_owned());
        let webhook_dummy_api_client = WebhookDummyApiClient::new(Arc::new(webhook_conf));

        LINE {
            channel_access_token_api_client,
            insight_api_client,
            liff_api_client,
            manage_audience_api_client,
            messaging_api_client,
            module_api_client,
            module_attach_api_client,
            shop_api_client,
            webhook_dummy_api_client,
        }
    }

    fn create_hyper_client() -> Client<Connector,String> {
        let https = HttpsConnector::new();
        Client::builder(TokioExecutor::new())
            .pool_idle_timeout(std::time::Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .build::<_, String>(https)
    }
}