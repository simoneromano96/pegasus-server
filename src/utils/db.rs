use wither::{
	self,
	mongodb::{Client, Database},
};

use crate::configuration::APP_CONFIG;

pub async fn init_database() -> Database {
	let db = Client::with_uri_str(&APP_CONFIG.mongo.uri)
		.await
		.expect("Connection error!")
		.database(&APP_CONFIG.mongo.database);

	db
}
