use super::BuildConfig;
use camino::Utf8PathBuf;
use color_eyre::{eyre::eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub release: bool,
    #[serde(default)]
    pub build: BuildConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            release: false,
            build: Default::default(),
        }
    }
}

impl Config {
    pub fn load(root_path: Utf8PathBuf, leptos_path: &Option<Utf8PathBuf>) -> Result<Self> {
        let config_file = if let Some(leptos_path) = leptos_path {
            let config = root_path.join(leptos_path);
            config.is_file().then_some(config)
        } else {
            ["Leptos.toml", "leptos.toml"]
                .into_iter()
                .map(|file| root_path.join(file))
                .find(|path| path.is_file())
        };

        let Some(leptos_config_file) = config_file else {
            return Result::Ok(Config::default());
        };

        toml::from_str::<Config>(&std::fs::read_to_string(&leptos_config_file)?).map_err(|err| {
            eyre!("Failed to parse Leptos.toml at {leptos_config_file:?}: {err}").into()
        })
    }
}
