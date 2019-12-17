#[macro_use]
extern crate log;

use std::time::Duration;

mod alert;
mod config;
mod fetch;
mod post;
mod utils;
mod webhook;

use config::{ConfigLoadError, StaticConfig};
use post::PrePosts;
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
    match utils::init_file_if_not_exists::<PrePosts>(&config.files.previous_posts) {
        Err(FileInitError::IO(e)) => {
            panic_error(&format!("Unable to open or create posts file: {:?}", e))
        }
        Err(FileInitError::JSON(e)) => {
            panic_error(&format!("Unable to write json to file: {:?}", e))
        }
        _ => {}
    };

    // Run a loop forever
    loop {
        match fetch::fetch_new_posts(&config) {
            Err(e) => error!("Failed fetching new posts: {:?}", e),
            Ok(posts) => {
                if let Some(posts) = posts {
                    // Alert about them
                    info!(
                        "New Alerts:\n{:#?}",
                        posts.iter().map(|x| &x.title).collect::<Vec<_>>()
                    );

                    for post in posts {
                        for webhook in &config.webhooks.discord[..1] {
                            // FIXME:
                            match alert::alert_discord(webhook, &post) {
                                Ok(_) => trace!(
                                    r#"Successfully alerted discord webhook "{}" for post "{}""#,
                                    webhook,
                                    post.title
                                ),
                                Err(e) => {
                                    error!(r#"Failed to alert discord webhook "{}" for post "{}": {:?}"#, webhook, post.title, e)
                                }
                            }
                        }
                    }
                }
            }
        };

        std::thread::sleep(Duration::from_secs(5));
    }
}
