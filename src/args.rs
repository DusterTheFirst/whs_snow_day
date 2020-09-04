use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[repr(C)]
/// WHS Urgent Alerts - Service to send push notifications about urgent alerts for wellesley public schools
pub struct CommandLineArgs {
    #[structopt(long, short, default_value = "Config.toml")]
    /// the path to the configration file
    pub config: PathBuf,
    #[structopt(long, default_value = "db")]
    /// the path to database file to store information about previous posts
    pub db: PathBuf,
    #[structopt(short, long)]
    /// Activate debug mode
    pub debug: bool,
    #[structopt(short, long, default_value = "600000")]
    /// The speed in which to check the endpoints for change (in miliseconds)
    pub update_speed: u32,
}
