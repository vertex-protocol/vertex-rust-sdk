use vertex_sdk::eip712_structs::to_bytes32;
use vertex_sdk::prelude::*;
use vertex_sdk::utils::private_key::private_key;

#[tokio::main]
async fn main() {
    // client with no signer
    let mut client = VertexClient::new(ClientMode::SepoliaTest);

    // client with signing enabled
    client = client.with_signer(private_key()).await.unwrap();

    let address = client.address().unwrap();
    let default_subaccount = to_bytes32(address.into(), "default");
    let client_subaccount = client.subaccount().unwrap();
    assert_eq!(client_subaccount, default_subaccount);

    // set custom subaccount name
    client = client.with_subaccount_name("bob");

    // use a custom [u8; 12] instead of a string
    client = client.with_subaccount_name_bytes([0; 12]);

    // set custom endpoint urls
    client = client.with_gateway_url("https://custom_url.com".to_string());
    client = client.with_archive_url("https://custom_url.com".to_string());
    client = client.with_trigger_url("https://custom_url.com".to_string());

    client.subaccount().unwrap();
}
