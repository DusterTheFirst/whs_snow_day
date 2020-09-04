#[macro_use]
extern crate log;

use std::panic::{self, PanicInfo};
use std::time::Duration;
use structopt::StructOpt;

mod alert;
mod args;
mod config;
mod fetch;
mod post;
mod utils;
mod webhook;

use args::CommandLineArgs;
use config::{ConfigLoadError, StaticConfig};
use post::PrePosts;
use utils::FileInitError;

use colored::*;

#[tokio::main]
async fn main() -> ! {
    // Load .env
    utils::load_dotenv();

    // Init logger
    pretty_env_logger::init();

    panic::set_hook(Box::new(|info: &PanicInfo| {
        eprintln!("{} {}", "Fatal error encountered:".red(), info);
    }));

    let args = CommandLineArgs::from_args();
    let db = match sled::Config::default()
        .path(&args.db)
        .create_new(!args.db.exists())
        .open()
    {
        Ok(db) => db,
        Err(e) => panic!("Failed to read database file: {0} {0:?}", e),
    };

    let posts = db.open_tree("posts").unwrap();

    posts.insert("penis", "args").unwrap();
    posts
        .insert("args", bincode::serialize(&args).unwrap())
        .unwrap();

    dbg!(&args);
    dbg!(bincode::deserialize::<CommandLineArgs>(
        &posts.get("args").unwrap().unwrap()
    ));

    // Load the config
    let config = match StaticConfig::load_from_file(&args.config) {
        Err(ConfigLoadError::IO(e)) => panic!("Unable to open or read config file: {:?}", e),
        Err(ConfigLoadError::TOML(e)) => panic!("Unable to parse toml file: {:?}", e),
        Ok(config) => config,
    };

    dbg!(&config);

    // Create the file
    match utils::init_file_if_not_exists::<PrePosts>(&config.files.previous_posts) {
        Err(FileInitError::IO(e)) => panic!("Unable to open or create posts file: {:?}", e),
        Err(FileInitError::JSON(e)) => panic!("Unable to write json to file: {:?}", e),
        _ => {}
    };

    // Run a loop forever
    loop {
        match fetch::fetch_new_posts(&config).await {
            Err(e) => error!("Failed fetching new posts: {}", e),
            Ok(posts) => {
                if let Some(posts) = posts {
                    // Alert about them
                    info!(
                        "New Alerts:\n{:#?}",
                        posts.iter().map(|x| &x.title).collect::<Vec<_>>()
                    );

                    #[cfg(not(debug_assertions))]
                    let webhooks = &config.webhooks.discord;
                    #[cfg(debug_assertions)]
                    let webhooks = &config.webhooks.discord[..1];

                    for post in posts {
                        for webhook in webhooks {
                            match alert::alert_discord(webhook, &post).await {
                                Ok(_) => trace!(
                                    r#"Successfully alerted discord webhook "{}" for post "{}""#,
                                    webhook,
                                    post.title
                                ),
                                Err(e) => error!(
                                    r#"Failed to alert discord webhook "{}" for post "{}": {}"#,
                                    webhook, post.title, e
                                ),
                            }
                        }
                    }
                }
            }
        };

        std::thread::sleep(Duration::from_secs(60));
    }
}
