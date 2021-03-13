use wither::{
	self,
	mongodb::{Client, Database},
	Model,
};

use crate::{configuration::APP_CONFIG, graphql::User};

pub async fn init_database() -> Database {
	let db = Client::with_uri_str(&APP_CONFIG.mongo.uri)
		.await
		.expect("Connection error!")
		.database(&APP_CONFIG.mongo.database);

	User::sync(&db).await.expect("Could not create user indexes!");

	db
}
