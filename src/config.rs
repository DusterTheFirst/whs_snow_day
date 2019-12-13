use serde::Deserialize;

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