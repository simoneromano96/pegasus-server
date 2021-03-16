use std::env;

use actix_web::cookie::Key;
use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
  pub static ref APP_CONFIG: Settings = Settings::init_config();
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MongoConfig {
  /// DB Connection URI
  pub uri: String,
  /// DB Name
  pub database: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerConfig {
  pub port: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SameSite {
  /// The "Strict" `SameSite` attribute.
  Strict,
  /// The "Lax" `SameSite` attribute.
  Lax,
  /// The "None" `SameSite` attribute.
  None,
}

impl Into<actix_web::cookie::SameSite> for SameSite {
  fn into(self) -> actix_web::cookie::SameSite {
    match self {
      SameSite::Strict => actix_web::cookie::SameSite::Strict,
      SameSite::Lax => actix_web::cookie::SameSite::Lax,
      SameSite::None => actix_web::cookie::SameSite::None,
    }
  }
}
/// Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
#[derive(Serialize, Deserialize)]
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
  pub maxage: u32,
  /// Cookie `SameSite`, Controls whether a cookie is sent with cross-origin requests
  ///
  /// Can be `Strict`, `Lax`, `None`, if `None` `Secure` must be true
  pub samesite: SameSite,
  /// Cookie encryption secret
  pub secret: String,
  /// Cookie encryption key derived from secret
  #[serde(skip)]
  pub key: Option<Key>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedisConfig {
  /// Redis connection URI
  pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerConfig {
  /// What should the (terminal) logger print
  pub level: String,
  /// File logger path output
  pub path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
  /// Logger configuration
  pub logger: LoggerConfig,
  /// Mongo database configuration
  pub mongo: MongoConfig,
  /// Some server configuration
  pub server: ServerConfig,
  /// Cookie and session configuration
  pub cookie: CookieConfig,
  /// Redis configuration
  pub redis: RedisConfig,
}

impl Settings {
  fn init_config() -> Self {
    // Start config
    let mut s = Config::default();

    // Create a path
    let mut config_file_path = env::current_dir().expect("Cannot get current path");

    // Get current RUN_MODE, should be: development/production
    let current_env = env::var("RUN_MODE").unwrap_or_else(|_| String::from("development"));

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
    let mut r: Settings = s.try_into().expect("Configuration error");

    let key = Key::from(r.cookie.secret.as_bytes());

    r.cookie.key = Some(key);

    r
  }
}
