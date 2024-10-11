use std::collections::HashMap;
use eyre::Result;
use reqwest::{Client, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utils::response::VertexRestResponse;

#[derive(Clone)]
pub struct RestClient {
    client: Client,
    headers: HashMap<String, Vec<String>>,
}

impl Default for RestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RestClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder().danger_accept_invalid_certs(true).build().expect("Build reqwest client"),
            headers: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.entry(key).or_default().push(value);
        self
    }

    pub async fn get_request<R: DeserializeOwned + Send>(&self, url: &str) -> Result<R> {
        request(self.append_headers(self.client.get(url))).await
    }

    pub async fn post_request<T: Serialize, R: DeserializeOwned + Send>(
        &self,
        url: &str,
        payload: &T,
    ) -> Result<R> {
        request(self.append_headers(self.client.post(url)).json(payload)).await
    }

    pub async fn debug_request<T: Serialize>(&self, url: &str, payload: &T) -> Result<()> {
        debug_request(self.append_headers(self.client.post(url)).json(payload)).await
    }

    fn append_headers(&self, mut builder: RequestBuilder) -> RequestBuilder {
        for (key, values) in &self.headers {
            for value in values {
                builder = builder.header(key, value);
            }
        }
        builder
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
