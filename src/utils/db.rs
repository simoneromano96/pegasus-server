use wither::{
    mongodb::{Client, Database},
    Model,
};

use crate::configuration::APP_CONFIG;
use crate::graphql::User;

/// Initialise DB connection and syncs indexes
/// Panics if the DB is not available or couldn't sync indexes
pub async fn init_database() -> Database {
    // Create DB connection
    let db = Client::with_uri_str(&APP_CONFIG.database.uri)
        .await
        .expect("Could not connect to the db")
        .database(&APP_CONFIG.database.name);

    // Sync indexes
    User::sync(&db).await.expect("Could not sync user indexes");
    db
}
