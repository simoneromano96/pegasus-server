use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wither::{
  bson::{doc, oid::ObjectId},
  mongodb::Database,
  prelude::*,
  WitherError,
};

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
  pub notes: Option<String>
}
