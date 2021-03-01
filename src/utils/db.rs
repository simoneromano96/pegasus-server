use wither::{
    mongodb::{Client, Database},
    Model,
};

use crate::graphql::User;

/// Initialise DB connection and syncs indexes
/// Panics if the DB is not available or couldn't sync indexes
pub async fn init_database() -> Database {
    // Create DB connection
    let db = Client::with_uri_str("mongodb://root:example@localhost:27017/")
        .await
        .expect("Could not connect to the db")
        .database("mydb");

    // Sync indexes
    User::sync(&db).await.expect("Could not sync user indexes");
    db
}
