use eyre::Result;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utils::response::VertexRestResponse;

#[derive(Clone)]
pub struct RestClient {
    client: Client,
}

impl RestClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
    pub async fn get_request<R: DeserializeOwned + Send>(&self, url: &str) -> Result<R> {
        request(self.client.get(url)).await
    }

    pub async fn post_request<T: Serialize, R: DeserializeOwned + Send>(
        &self,
        url: &str,
        payload: &T,
    ) -> Result<R> {
        request(self.client.post(url).json(payload)).await
    }
}

async fn request<R: DeserializeOwned + Send>(request: RequestBuilder) -> Result<R> {
    let response = request.send().await?;
    let response_data: VertexRestResponse<R> = response.json().await?;
    response_data.extract_response()
}
