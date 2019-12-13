#[macro_use]
extern crate log;

use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::time::Duration;

mod config;
mod post;

use config::StaticConfig;
use post::{Post, PrePosts};

fn main() {
    // Load .env
    load_dotenv();

    // Init logger
    pretty_env_logger::init();

    // Load the config
    let config: StaticConfig = toml::from_str(&fs::read_to_string("Config.toml").unwrap()).unwrap();
    trace!("{:#?}", config);

    // Create the file
    if File::open(&config.files.previous_posts).is_err() {
        let file = File::create(&config.files.previous_posts).unwrap();

        serde_json::to_writer(
            file,
            &PrePosts {
                hash: 0,
                posts: vec![],
            },
        )
        .unwrap()
    }

    // Run a loop forever
    loop {
        trace!(
            r#"Making request to "{}""#,
            config.endpoints.no_school_posts
        );

        // Get the posts
        let posts: Vec<Post> = reqwest::get(&config.endpoints.no_school_posts)
            .unwrap()
            .json()
            .unwrap();
        trace!("{:#?}", posts);

        // Calculate hash
        let mut hasher = DefaultHasher::new();
        posts.hash(&mut hasher);
        let hash = hasher.finish();

        // Read previous posts
        let preposts: PrePosts =
            serde_json::from_reader(File::open(&config.files.previous_posts).unwrap()).unwrap();

        if hash != preposts.hash {
            // Alert about them
            info!(
                "New Alerts:\n{:#?}",
                posts[0..posts.len() - preposts.posts.len()]
                    .iter()
                    .map(|x| &x.title)
                    .collect::<Vec<_>>()
            );

            // Update the file
            fs::write(
                &config.files.previous_posts,
                serde_json::to_vec_pretty(&PrePosts { hash, posts }).unwrap(),
            )
            .unwrap();
        } else {
            trace!("no change");
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}

fn load_dotenv() {
    #[cfg(debug_assertions)]
    dotenv::from_filename(".debug.env").ok();

    #[cfg(not(debug_assertions))]
    dotenv::from_filename(".release.env").ok();
}
