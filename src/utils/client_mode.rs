use crate::utils::deployment::Deployment;
use std::fs;

#[derive(Clone, PartialEq)]
pub enum ClientMode {
    Prod,
    SepoliaTest,
    Local,
}

impl ClientMode {
    pub fn default_gateway_url(&self) -> String {
        match self {
            Self::Local => "http://localhost".to_string(),
            _ => {
                let envtag = self.vertex_envtag();
                format!("https://gateway.{envtag}.vertexprotocol.com/v1")
            }
        }
    }

    pub fn default_trigger_url(&self) -> String {
        match self {
            Self::Local => "http://localhost:8080/v1".to_string(),
            _ => {
                let envtag = self.vertex_envtag();
                format!("https://trigger.{envtag}.vertexprotocol.com/v1")
            }
        }
    }

    pub fn default_archive_url(&self) -> String {
        match self {
            Self::Local => "http://localhost:8000/v1".to_string(),
            _ => {
                let envtag = self.vertex_envtag();
                format!("https://archive.{envtag}.vertexprotocol.com/v1")
            }
        }
    }

    pub fn vertex_envtag(&self) -> String {
        match self {
            Self::Local => "local",
            Self::SepoliaTest => "sepolia-test",
            Self::Prod => "prod",
        }
        .to_string()
    }

    pub fn deployment(&self) -> Deployment {
        let network = self.vertex_envtag();
        let package_dir = env!("CARGO_MANIFEST_DIR");
        let vertex_utils_relative = "src/vertex_utils";
        let deployment =
            format!("{package_dir}/{vertex_utils_relative}/configs/{network}/deployment.json");
        let deployment = fs::read_to_string(deployment).unwrap();
        let deployment: Deployment = serde_json::from_str(&deployment).unwrap();
        deployment
    }
}
