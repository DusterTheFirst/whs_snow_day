#[macro_use]
extern crate log;

use std::fs;
use std::time::Duration;

mod config;
mod fetch;
mod post;
mod utils;

use config::{ConfigLoadError, StaticConfig};
use post::{PrePosts};
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
    let postsfile: fs::File =
        match utils::init_file_if_not_exists::<PrePosts>(&config.files.previous_posts) {
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
        match fetch::fetch_new_posts(&config, &postsfile) {
            Err(e) => {
                error!("Failed fetching new posts: {:?}", e)
            }
            Ok(posts) => {
                if let Some(posts) = posts {
                    // Alert about them
                    info!(
                        "New Alerts:\n{:#?}",
                        posts
                            .iter()
                            .map(|x| &x.title)
                            .collect::<Vec<_>>()
                    );
                }
            }
        };

        std::thread::sleep(Duration::from_secs(5));
    }
}
