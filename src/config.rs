use serde::Deserialize;
use toml::de::Error as TomlError;
use std::io::Error as IOError;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct StaticConfig {
    pub endpoints: EndpointsConfig,
    pub files: FilesConfig
}

#[derive(Debug, Deserialize)]
pub struct EndpointsConfig {
    pub no_school_posts: String
}

#[derive(Debug, Deserialize)]
pub struct FilesConfig {
    pub previous_posts: String
}

impl StaticConfig {
    #[inline]
    pub fn load_from_file(filename: &str) -> Result<Self, ConfigLoadError> {
        Ok(toml::from_str(&fs::read_to_string(filename)?)?)
    }
}

#[derive(Debug)]
pub enum ConfigLoadError {
    TOML(TomlError),
    IO(IOError)
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