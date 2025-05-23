use crate::utils::deployment::Deployment;
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;

pub static CONFIGS: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/vertex_utils/configs");
#[derive(Clone, PartialEq)]
pub enum ClientMode {
    Prod,
    BlastProd,
    MantleProd,
    SepoliaTest,
    BlastTest,
    MantleTest,
    SeiTest,
    SeiProd,
    BaseTest,
    BaseProd,
    SonicTest,
    SonicProd,
    AbstractTest,
    AbstractProd,
    BeraProd,
    AvaxTest,
    AvaxProd,
    XrplTest,
    KatanaTest,
    Local,
    LocalAlt,
}

impl ClientMode {
    pub fn default_gateway_url(&self) -> String {
        let envtag = self.vertex_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://gateway.{envtag}.vertexprotocol.com:80/v1")
            }
            _ => {
                format!("https://gateway.{envtag}.vertexprotocol.com/v1")
            }
        }
    }

    pub fn default_trigger_url(&self) -> String {
        let envtag = self.vertex_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://trigger.{envtag}.vertexprotocol.com:8080/v1")
            }
            _ => {
                format!("https://trigger.{envtag}.vertexprotocol.com/v1")
            }
        }
    }

    pub fn default_archive_url(&self) -> String {
        let envtag = self.vertex_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://archive.{envtag}.vertexprotocol.com:8000/v1")
            }
            _ => {
                format!("https://archive.{envtag}.vertexprotocol.com/v1")
            }
        }
    }

    pub fn default_gateway_ws_url(&self) -> String {
        let envtag = self.vertex_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("ws://gateway.{envtag}.vertexprotocol.com:80/ws")
            }
            _ => {
                format!("wss://gateway.{envtag}.vertexprotocol.com/ws")
            }
        }
    }

    pub fn private_gateway_ws_url(&self) -> String {
        let envtag = self.vertex_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("ws://{envtag}-mm.vertexprotocol-backend.com:80/ws")
            }
            _ => {
                format!("wss://{envtag}-mm.vertexprotocol-backend.com/ws")
            }
        }
    }

    pub fn private_gateway_subscription_url(&self) -> String {
        let envtag = self.vertex_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("ws://{envtag}-mm.vertexprotocol-backend.com:80/subscribe")
            }
            _ => {
                format!("wss://{envtag}-mm.vertexprotocol-backend.com/subscribe")
            }
        }
    }

    pub fn private_gateway_url(&self) -> String {
        let envtag = self.vertex_envtag();
        match self {
            Self::Local | Self::LocalAlt => {
                format!("http://{envtag}-mm.vertexprotocol-backend.com:80")
            }
            _ => {
                format!("https://{envtag}-mm.vertexprotocol-backend.com")
            }
        }
    }

    pub fn vertex_envtag(&self) -> String {
        match self {
            Self::Local => "local",
            Self::LocalAlt => "local-alt",
            Self::SepoliaTest => "sepolia-test",
            Self::MantleTest => "mantle-test",
            Self::BlastTest => "blast-test",
            Self::SeiTest => "sei-test",
            Self::BaseTest => "base-test",
            Self::SonicTest => "sonic-test",
            Self::AbstractTest => "abstract-test",
            Self::Prod => "prod",
            Self::BlastProd => "blast-prod",
            Self::MantleProd => "mantle-prod",
            Self::SeiProd => "sei-prod",
            Self::BaseProd => "base-prod",
            Self::SonicProd => "sonic-prod",
            Self::AbstractProd => "abstract-prod",
            Self::BeraProd => "bera-prod",
            Self::AvaxTest => "avax-test",
            Self::AvaxProd => "avax-prod",
            Self::XrplTest => "xrpl-test",
            Self::KatanaTest => "katana-test",
        }
        .to_string()
    }

    pub fn from_envtag(envtag: &str) -> Self {
        match envtag {
            "local" => Self::Local,
            "local-alt" => Self::LocalAlt,
            "sepolia-test" => Self::SepoliaTest,
            "blast-test" => Self::BlastTest,
            "sei-test" => Self::SeiTest,
            "base-test" => Self::BaseTest,
            "sonic-test" => Self::SonicTest,
            "abstract-test" => Self::AbstractTest,
            "avax-test" => Self::AvaxTest,
            "xrpl-test" => Self::XrplTest,
            "katana-test" => Self::KatanaTest,
            "prod" => Self::Prod,
            "blast-prod" => Self::BlastProd,
            "mantle-test" => Self::MantleTest,
            "mantle-prod" => Self::MantleProd,
            "sei-prod" => Self::SeiProd,
            "base-prod" => Self::BaseProd,
            "sonic-prod" => Self::SonicProd,
            "abstract-prod" => Self::AbstractProd,
            "bera-prod" => Self::BeraProd,
            "avax-prod" => Self::AvaxProd,
            _ => panic!("Unknown envtag: {}", envtag),
        }
    }

    pub fn deployment(&self) -> Deployment {
        let network = self.vertex_envtag();
        let deployment_path = Path::new(&network).join("deployment.json");

        // Order of loading
        // 1. try ENVVAR override
        // 2. try locally configured directory
        // 3. use deployment.json included in binary
        let deployment_str = match std::env::var("VERTEX_CONFIGS_DIR") {
            Ok(path) => {
                let deployment_path = Path::new(&path).join(&network).join("deployment.json");
                fs::read_to_string(&deployment_path).expect(&format!(
                    "Failed to read {}",
                    &deployment_path.to_string_lossy()
                ))
            }
            Err(_) => {
                let package_dir = env!("CARGO_MANIFEST_DIR");
                let default_configs_path = Path::new(package_dir)
                    .join("src/vertex_utils/configs")
                    .join(&network)
                    .join("deployment.json");

                fs::read_to_string(&default_configs_path).unwrap_or_else(|_| {
                    CONFIGS
                        .get_file(deployment_path.to_str().unwrap())
                        .expect("Deployment file not found")
                        .contents_utf8()
                        .expect("Failed to read deployment file")
                        .to_string()
                })
            }
        };

        serde_json::from_str(&deployment_str).unwrap()
    }
}
