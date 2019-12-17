use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::time::Duration;

use crate::config::{ConfigLoadError, StaticConfig};
use crate::post::{Post, PrePosts};
use crate::utils::{panic_error, FileInitError};

#[inline]
pub fn fetch_new_posts(config: StaticConfig, postsfile: File) -> Result<Error, Option<Vec<Post>>> {
    trace!(
        r#"Making request to "{}""#,
        config.endpoints.no_school_posts
    );

    let mut response = match reqwest::get(&config.endpoints.no_school_posts) {
        Err(e) => {
            warn!("Problem fetching the school posts: {:?}", e);

            return None;
        }
        Ok(response) => response,
    };

    // Get the posts
    let posts: Vec<Post> = match response.json() {
        Err(e) => {
            warn!("Problem parsing the school posts: {:?}", e);

            return None;
        }
        Ok(posts) => posts,
    };
    trace!("{:#?}", posts);

    // Calculate hash
    let mut hasher = DefaultHasher::new();
    posts.hash(&mut hasher);
    let hash = hasher.finish();

    // Parse previous posts
    let preposts: PrePosts = match serde_json::from_reader(&postsfile) {
        Err(e) => {
            error!("Unable to parse previous posts file: {:?}", e);

            return;
        }
        Ok(posts) => posts,
    };

    if hash != preposts.hash {
        // Alert about them
        info!(
            "New Alerts:\n{:#?}",
            posts[0..posts.len() - preposts.posts.len()]
                .iter()
                .map(|x| &x.title)
                .collect::<Vec<_>>()
        );

        // Serialize the file contents
        let contents = match serde_json::to_vec_pretty(&PrePosts { hash, posts }) {
            Err(e) => {
                error!("Unable to serialze new posts to json: {:?}", e);
                return;
            }
            Ok(c) => c,
        };

        // Update the file
        match fs::write(&config.files.previous_posts, contents) {
            Err(e) => {
                error!("Unable to serialze new posts to json: {:?}", e);
                return;
            }
            Ok(()) => trace!("Updated previous posts"),
        }
    } else {
        trace!("No change");
    }
}
