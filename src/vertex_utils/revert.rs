use ethers_contract::{ContractError, EthError};
use ethers_providers::Middleware;

pub fn parse_provider_error<M: Middleware>(error: ContractError<M>) -> String {
    match error {
        ContractError::Revert(ref data) => {
            let msg = String::decode_with_selector(data.to_vec().as_slice()).unwrap_or_else(|| {
                println!("failed to decode revert data: {:?}", error);
                String::default()
            });
            format!("execution reverted: {}", msg)
        }
        _ => error.to_string(),
    }
}
