use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{
    Http, HttpClientError, HttpRateLimitRetryPolicy, JsonRpcClient, JsonRpcError, Middleware,
    Provider, RetryClient,
};
use ethers_signers::Signer;
use eyre::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub type VertexProvider<S> =
    SignerMiddleware<Provider<RetryClient<Http>>, S>;

pub type VertexEnsembleProvider<S> =
    SignerMiddleware<Provider<RetryClient<HttpEnsemble>>, S>;

/// Client that uses a different node url on failure
#[derive(Debug)]
pub struct HttpEnsemble {
    pub providers: Vec<Http>,
}

impl HttpEnsemble {
    pub fn new(node_urls: &Vec<String>) -> Result<Self> {
        let mut providers = vec![];
        for url in node_urls.iter() {
            providers.push(Http::from_str(url)?)
        }
        Ok(HttpEnsemble { providers })
    }

    pub fn new_provider(node_urls: &Vec<String>) -> Result<Provider<Self>> {
        let http_ensemble = Self::new(node_urls)?;
        Ok(Provider::new(http_ensemble))
    }

    pub fn new_retry_provider(node_urls: &Vec<String>) -> Result<Provider<RetryClient<Self>>> {
        let http_ensemble = Self::new(node_urls)?;
        Ok(Provider::new(RetryClient::new(
            http_ensemble,
            Box::new(HttpRateLimitRetryPolicy),
            15,
            500,
        )))
    }

    pub async fn new_vertex_provider<S: Signer>(
        node_urls: &Vec<String>,
        signer: S,
    ) -> Result<Arc<VertexEnsembleProvider<S>>> {
        let provider = HttpEnsemble::new_retry_provider(node_urls)?;
        let chain_id = provider.get_chainid().await?;
        let signer = signer.with_chain_id(chain_id.as_u64());
        Ok(Arc::new(SignerMiddleware::new(
            provider.interval(Duration::from_millis(500)),
            signer,
        )))
    }
}

#[async_trait]
impl JsonRpcClient for HttpEnsemble {
    type Error = HttpClientError;

    async fn request<A, R>(&self, method: &str, params: A) -> Result<R, Self::Error>
    where
        A: Debug + Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let mut last_res = Err(Self::Error::JsonRpcError(JsonRpcError {
            code: 0,
            message: "list of providers was empty".to_string(),
            data: None,
        }));

        for (i, p) in self.providers.iter().enumerate() {
            if i != 0 {
                if let Err(e) = &last_res {
                    let err_str = e.to_string();
                    // gas data returned in revert msg
                    if err_str.contains(": G ") {
                        break;
                    }
                    println!(
                        "http_ensemble: using backup node_url: {}\n{err_str}",
                        p.url(),
                    );
                }
            }
            last_res = p.request(method, &params).await;
            if last_res.is_ok() {
                break;
            }
        }
        last_res
    }
}
