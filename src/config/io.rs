use define_config::{Serialize, Deserialize};

use config::{LatestConfig, Upgrade};

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

#[allow(dead_code)]
/// Loads config file and upgrades it to latest format.
/// If there is an error loading file, config is overwritten with
/// the defaults.
pub fn load_config() -> LatestConfig
    where LatestConfig: Upgrade + Deserialize
{
    let mut file = get_config_file();
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read config file");

    match LatestConfig::from_str(&contents) {
        Ok(config) => config,
        Err(_) => {
            drop(file);
            panic!("{}", config);
            // let config = LatestConfig::default();
            // write_config(&config);
            // config
        }
    }
}

#[allow(dead_code)]
/// Panics on error writing to file
pub fn write_config<S: Serialize + Display>(config: &S) {
    let mut file = get_config_file();
    let string = config.to_string();
    let _ = file.write_all(string.as_bytes());
    file.sync_all().expect("Could not save config file");
}

#[allow(dead_code)]
/// Creates config file if missing.
/// Returns a handle to the config file.
///
/// Panics when unable to load file
fn get_config_file() -> File {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("./config.yml");

    match file {
        Ok(file) => file,
        Err(err) => panic!("Error loading config file: {}", err.description()),
    }
}
