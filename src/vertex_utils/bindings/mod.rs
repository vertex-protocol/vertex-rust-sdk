pub mod clearinghouse;
pub mod clearinghouse_liq;
pub mod endpoint;
pub mod gas_info;
pub mod mock_erc20;
pub mod offchain_exchange;
pub mod perp_engine;
pub mod querier;
pub mod spot_engine;
pub mod token;

pub mod verifier;

use ethers_contract::Abigen;

struct BindingGenerator {
    input_root: String,
    output_root: String,
}

impl BindingGenerator {
    pub fn new(input_root: &str, output_root: &str) -> Self {
        BindingGenerator {
            input_root: input_root.to_string(),
            output_root: output_root.to_string(),
        }
    }
    pub fn generate(&self, name: &str, input_path: &str, output_path: &str) {
        let input_path = format!("{}/{}", self.input_root, input_path);
        let output_path = format!("{}/{}", self.output_root, output_path);
        Abigen::new(name, input_path)
            .unwrap()
            .emit_cargo_directives(true)
            // .add_derive("serde::Serialize")
            // .unwrap()
            // .add_derive("serde::Deserialize")
            // .unwrap()
            // .add_derive("rkyv::Serialize")
            // .unwrap()
            // .add_derive("rkyv::Deserialize")
            // .unwrap()
            // .add_derive("rkyv::Archive")
            // .unwrap()
            .generate()
            .unwrap()
            .write_to_file(output_path)
            .unwrap();
    }
}

pub fn create_bindings(source_dir: String, output_dir: String) {
    let generator = BindingGenerator::new(&source_dir, &output_dir);
    generator.generate("MockERC20", "MockERC20.json", "mock_erc20.rs");
    generator.generate("GasInfo", "GasInfo.json", "gas_info.rs");
    generator.generate(
        "OffchainExchange",
        "FOffchainExchange.json",
        "offchain_exchange.rs",
    );
    generator.generate("PerpEngine", "FPerpEngine.json", "perp_engine.rs");
    generator.generate("SpotEngine", "FSpotEngine.json", "spot_engine.rs");
    generator.generate("Clearinghouse", "FClearinghouse.json", "clearinghouse.rs");
    generator.generate(
        "ClearinghouseLiq",
        "ClearinghouseLiq.json",
        "clearinghouse_liq.rs",
    );
    generator.generate("Endpoint", "FEndpoint.json", "endpoint.rs");
    generator.generate("Verifier", "Verifier.json", "verifier.rs");
    generator.generate("Querier", "FQuerier.json", "querier.rs");

    // EngineToken because there is a collision with Token
    generator.generate("EngineToken", "FToken.json", "token.rs");
}
