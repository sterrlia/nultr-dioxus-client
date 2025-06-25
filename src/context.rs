use std::sync::Arc;

use dioxus::prelude::*;
use nultr_client_lib::config;
use rust_api_kit::http::client::HttpClient;

#[derive(Clone)]
pub struct Services {
    pub http_client: Arc<HttpClient>,
}

impl Default for Services {
    fn default() -> Self {
        let http_url = config::get_variables().http_url.clone();
        let http_client = Arc::new(HttpClient::new(http_url));

        Self { http_client }
    }
}

pub fn init_context() {
    use_context_provider(|| Services::default());
}
