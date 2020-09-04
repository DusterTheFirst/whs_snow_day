use serde::Deserialize;
use std::fs;
use std::io::Error as IOError;
use std::path::Path;
use toml::de::Error as TomlError;

#[derive(Debug, Deserialize)]
pub struct StaticConfig {
    pub endpoints: EndpointsConfig,
    pub files: FilesConfig,
    pub webhooks: WebhooksConfig,
}

#[derive(Debug, Deserialize)]
pub struct EndpointsConfig {
    pub no_school_posts: String,
}

#[derive(Debug, Deserialize)]
pub struct FilesConfig {
    pub previous_posts: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhooksConfig {
    pub discord: Vec<String>,
}

impl StaticConfig {
    pub fn load_from_file<S: AsRef<Path>>(filename: S) -> Result<Self, ConfigLoadError> {
        Ok(toml::from_str(&fs::read_to_string(filename)?)?)
    }
}

#[derive(Debug)]
pub enum ConfigLoadError {
    TOML(TomlError),
    IO(IOError),
}

impl From<IOError> for ConfigLoadError {
    fn from(e: IOError) -> Self {
        Self::IO(e)
    }
}

impl From<TomlError> for ConfigLoadError {
    fn from(e: TomlError) -> Self {
        Self::TOML(e)
    }
}
