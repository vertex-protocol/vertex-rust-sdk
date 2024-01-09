use eyre::eyre;

const PRIVATE_KEY_ENVVAR: &str = "RUST_SDK_PRIVATE_KEY";

pub fn private_key() -> String {
    dotenv::dotenv().ok();
    match std::env::var(PRIVATE_KEY_ENVVAR) {
        Ok(key) => Ok(key),
        Err(_) => Err(eyre!("{PRIVATE_KEY_ENVVAR} envvar not found")),
    }
    .unwrap()
}
