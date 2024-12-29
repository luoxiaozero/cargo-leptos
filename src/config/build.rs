use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildConfig {
    #[serde(default)]
    pub output_name: Option<String>,
    #[serde(default = "default_output_dir")]
    pub output_dir: String,

    #[serde(default = "BuildModeConfig::csr")]
    pub csr: BuildModeConfig,
    #[serde(default = "BuildModeConfig::ssr")]
    pub ssr: BuildModeConfig,
    #[serde(default = "BuildModeConfig::hydrate")]
    pub hydrate: BuildModeConfig,
}

fn default_output_dir() -> String {
    "target/site".to_string()
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_name: Default::default(),
            output_dir: default_output_dir(),
            csr: BuildModeConfig::csr(),
            ssr: BuildModeConfig::ssr(),
            hydrate: BuildModeConfig::hydrate(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct BuildModeConfig {
    pub lib: bool,
    pub args: Vec<String>,
    pub target: String,
    pub default_features: bool,
    pub features: Vec<String>,
}

impl BuildModeConfig {
    fn csr() -> Self {
        Self {
            lib: Default::default(),
            args: Default::default(),
            target: "wasm32-unknown-unknown".to_string(),
            default_features: Default::default(),
            features: Default::default(),
        }
    }

    fn ssr() -> Self {
        Self {
            lib: Default::default(),
            args: Default::default(),
            target: current_platform::CURRENT_PLATFORM.to_string(),
            default_features: Default::default(),
            features: Default::default(),
        }
    }

    fn hydrate() -> Self {
        Self {
            lib: true,
            args: Default::default(),
            target: "wasm32-unknown-unknown".to_string(),
            default_features: Default::default(),
            features: Default::default(),
        }
    }
}
