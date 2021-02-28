use std::env;

use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref APP_CONFIG: Settings = Settings::init_config();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoConfig {
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
    pub port: u16,
}

/// Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CookieConfig {
	/// Cookie `cookie-name`, any valid ASCII characters ex. session-id
    pub name: String,
	/// Cookie `Path`, ex. /
    pub path: String,
	/// Cookie `Domain`, ex. website.com
    pub domain: String,
	/// Cookie `Secure`, if true will be set and sent only on https
    pub secure: bool,
	/// Cookie `HttpOnly`, if true client-side js cannot read the cookie
    pub httponly: bool,
	/// Cookie `Max-Age`, Number of seconds until the cookie expires
    pub maxage: i64,
	/// Cookie `SameSite`, Controls whether a cookie is sent with cross-origin requests
	///
	/// Can be `Strict`, `Lax`, `None`, if `None` `Secure` must be true
	pub samesite: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
	pub debug: bool,
    pub database: MongoConfig,
    pub server: ServerConfig,
	pub cookie: CookieConfig,
}

impl Settings {
    fn init_config() -> Self {
		// Start config
		let mut s = Config::default();

		// Create a path
        let mut config_file_path = env::current_dir().expect("Cannot get current path");

        // Get current RUN_MODE, should be: development/production
        let current_env = env::var("RUN_MODE").unwrap_or(String::from("development"));

		// From current path add /environments
        config_file_path.push("environments");
        // Add RUN_MODE.yaml
		config_file_path.push(format!("{}.yaml", current_env));

        // Add in the current environment file
        // Default to 'development' env
        s.merge(File::from(config_file_path).required(false))
            .expect("Could not read file");

        // Add in settings from the environment
        // ex. APP_DEBUG=1 sets debug key, APP_DATABASE_URL sets database.url key
        s.merge(Environment::new().prefix("APP").separator("_"))
            .expect("Cannot get env");

        // Deserialize configuration
        let r: Settings = s.try_into().expect("Configuration error");

        r
    }
}
