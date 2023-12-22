use std::sync::OnceLock;

use crate::{errors::Error, errors::Result};

/// Returns the configuration for the application
/// in a once lock static variable that is loaded
/// from the environment variables
pub fn config() -> &'static Config{
    static CONFIG_INSTANCE: OnceLock<Config> = OnceLock::new();

    CONFIG_INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| {
            panic!("FATAL ERROR WHILE LOADING CONFIG:: {e:?}");
        })
    })
}

/// Stores the configuration for the application
/// in a once lock static variable
/// it is loaded from the environment variables
/// that can be present either in the environment
/// itself or in the .cargo/config.toml file
pub struct Config{
    // --static
    pub static_folder: String,
    pub version: String,
    pub main_port: String,
    pub database_url: String,
}

impl Config{
    fn load_from_env() -> Result<Config>{
        Ok(Config{
            static_folder: get_env("STATIC_FOLDER")?,
            version: get_env("VERSION")?,
            main_port: get_env("MAIN_SERVER_PORT")?,
            database_url: get_env("DATABASE_URL")?
        })
    }
}

fn get_env(name: &'static str) -> Result<String>{
    std::env::var(name).map_err(|_| Error::EnvMissing(name))
}