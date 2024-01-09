use ethers::core::k256;
use ethers_middleware::SignerMiddleware;
use ethers_providers::{Http, Provider, RetryClient};
use ethers_signers::Wallet;

pub type VertexProvider =
    SignerMiddleware<Provider<RetryClient<Http>>, Wallet<k256::ecdsa::SigningKey>>;
