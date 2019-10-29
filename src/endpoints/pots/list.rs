use super::Pots;
use crate::{endpoints::handle_response, into_future::IntoFuture, Result};
use reqwest::Client as HttpClient;
use std::future::Future;

/// An object representing a request to the Monzo API for a list of accounts
pub struct ListPots {
    request_builder: reqwest::RequestBuilder,
}

impl ListPots {
    pub(crate) fn new(http_client: &HttpClient, access_token: impl AsRef<str>) -> Self {
        let request_builder = http_client
            .get("https://api.monzo.com/accounts")
            .bearer_auth(access_token.as_ref());

        Self { request_builder }
    }

    /// Consume the request and return a response that will resolve to a list of
    /// pots
    pub async fn send(self) -> Result<Pots> {
        handle_response(self.request_builder).await
    }
}

impl IntoFuture for ListPots {
    type Output = Result<Pots>;
    type Future = impl Future<Output = Self::Output>;

    fn into_future(self) -> Self::Future {
        self.send()
    }
}
