use anyhow::Result;
use wither::{
  self,
  mongodb::{Client, Database},
  Model,
};

use crate::{configuration::APP_CONFIG, graphql::User};

pub async fn init_database() -> Result<Database> {
  let db = Client::with_uri_str(&APP_CONFIG.mongo.uri)
    .await?
    .database(&APP_CONFIG.mongo.database);

  User::sync(&db).await?;

  Ok(db)
}
