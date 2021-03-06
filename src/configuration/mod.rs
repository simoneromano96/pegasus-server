pub mod logger;

use std::env;

use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use slog::{info, Logger};
use sloggers::{
	terminal::{Destination, TerminalLoggerBuilder},
	types::Severity,
	Build,
};

lazy_static! {
	pub static ref APP_CONFIG: Settings = Settings::init();
}

#[derive(Debug, Deserialize)]
pub struct Settings {
	/// Enable some debugging logs
	pub debug: bool,
}

impl Settings {
	/// Initialize Settings
	pub fn init() -> Self {
		let mut s = Config::new();

		// Add in the current environment file
		// Default to 'development' env
		// Note that this file is _optional_
		let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

		// config/${RUN_MODE = development}.yaml
		s.merge(File::with_name(&format!("environments/{}.yaml", env)).required(true))
			.expect("Could not read configuration file");

		// Add in settings from the environment (with a prefix of APP)
		// Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
		s.merge(Environment::with_prefix("APP"))
			.expect("Could not read environment");

		s.try_into().expect("Could not create settings struc")
	}
}
