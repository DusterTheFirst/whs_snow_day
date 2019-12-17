#[macro_use]
extern crate log;

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Duration;

mod config;
mod post;
mod utils;
mod fetch;

use config::{ConfigLoadError, StaticConfig};
use post::{Post, PrePosts};
use utils::{panic_error, FileInitError};

fn main() {
    // Load .env
    utils::load_dotenv();

    // Init logger
    pretty_env_logger::init();

    // Load the config
    let config = match StaticConfig::load_from_file("Config.toml") {
        Err(ConfigLoadError::IO(e)) => {
            panic_error(&format!("Unable to open or read config file: {:?}", e))
        }
        Err(ConfigLoadError::TOML(e)) => {
            panic_error(&format!("Unable to parse toml file: {:?}", e))
        }
        Ok(config) => config,
    };
    trace!("{:#?}", config);

    // Create the file
    let postsfile: fs::File = match utils::init_file_if_not_exists::<PrePosts>(&config.files.previous_posts) {
        Err(FileInitError::IO(e)) => {
            panic_error(&format!("Unable to open or create posts file: {:?}", e))
        }
        Err(FileInitError::JSON(e)) => {
            panic_error(&format!("Unable to write json to file: {:?}", e))
        }
        Ok(f) => {
            info!(
                r#"File "{}" not found, creating it now."#,
                config.files.previous_posts
            );
            f
        }
    };

    // Run a loop forever
    loop {
        fetch::fetch_new_posts(config, postsfile);

        std::thread::sleep(Duration::from_secs(5));
    }
}
