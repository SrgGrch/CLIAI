use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub api_key: String,
    pub model_id: String,
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
}

fn default_endpoint() -> String {
    "https://openrouter.ai/api/v1".to_string()
}

// MARK: Public methods
impl Config {
    pub fn set_api_key(api_key: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let path = Config::config_path();

        let _config: Config = Config::get_config(&path)?;

        let new_config = Config {
            api_key: api_key.to_string(),
            .._config
        };

        let new_content = toml::to_string(&new_config)?;
        fs::write(&path, new_content)?;

        Ok(new_config)
    }

    pub fn set_model(model_id: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let path = Config::config_path();

        let _config: Config = Config::get_config(&path)?;

        let new_config = Config {
            model_id: model_id.to_string(),
            .._config
        };

        let new_content = toml::to_string(&new_config)?;
        fs::write(&path, new_content)?;

        Ok(new_config)
    }

    pub fn set_endpoint(endpoint: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let path = Config::config_path();
        let _config: Config = Config::get_config(&path)?;

        let new_config = Config {
            endpoint: endpoint.to_string(),
            .._config
        };

        let new_content = toml::to_string(&new_config)?;
        fs::write(&path, new_content)?;

        Ok(new_config)
    }

    pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
        let path = Config::config_path();
        let config: Config = Config::get_config(&path)?;

        Ok(config)
    }
}

// MARK: Private methods
impl Config {
    fn get_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(toml::from_str(&content)?)
        } else {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(path, "")?;

            Ok(Config::default())
        }
    }

    fn config_path() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "srggrch", "cliai")
            .expect("Cannot determine config directory");
        proj_dirs.config_dir().join("config.toml")
    }
}
