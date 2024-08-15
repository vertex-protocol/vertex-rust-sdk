use eyre::Result;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utils::response::VertexRestResponse;

#[derive(Clone)]
pub struct RestClient {
    client: Client,
}

impl Default for RestClient {
    fn default() -> Self {
        Self::new()
    }
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

    pub async fn debug_request<T: Serialize>(&self, url: &str, payload: &T) -> Result<()> {
        debug_request(self.client.post(url).json(payload)).await
    }
}

async fn request<R: DeserializeOwned + Send>(request: RequestBuilder) -> Result<R> {
    let response = request.send().await?;
    let response_data: VertexRestResponse<R> = response.json().await?;
    response_data.extract_response()
}

async fn debug_request(request: RequestBuilder) -> Result<()> {
    let response = request.send().await?;
    let response_data = response.text().await?;
    println!("response_data: {}", response_data);
    Ok(())
}
