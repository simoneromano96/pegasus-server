use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use thiserror::Error;
use wither::{
  bson::{doc, oid::ObjectId},
  mongodb::Database,
  prelude::*,
  WitherError,
};

use crate::graphql::User;

#[derive(Debug, Clone, Model, Serialize, Deserialize, SimpleObject)]
#[model()]
pub struct Account {
  /// This account ID
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  /// Account username, this can be, for example, an email, an actual username etc.
  pub username: String,
  /// Account password, this is optional, for example some sites require only sms, email, etc. login
  pub password: Option<String>,
  /// This account notes, can be any additional data
  pub notes: Option<String>,
}

pub async fn create_account(
  db: &Database,
  user: &User,
  username: String,
  password: Option<String>,
  notes: Option<String>,
) -> Result<Account> {
  let mut new_account = Account {
    id: None,
    username,
    password,
    notes,
  };

  new_account.save(db, None).await?;

  let account_id = new_account.id.as_ref().unwrap();

  let mut db_user = user.clone();
  db_user.account_ids.push(account_id.clone());
  db_user.save(db, None).await?;

  Ok(new_account)
}
