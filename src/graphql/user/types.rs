use async_graphql::{ComplexObject, Context, SimpleObject};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wither::{
  bson::{doc, oid::ObjectId},
  mongodb::Database,
  prelude::*,
  WitherError,
};

use crate::{
  graphql::Account,
  utils::{hash_password, verify_password, PasswordErrors},
};

#[derive(Error, Debug)]
pub enum UserErrors {
  #[error("{0}")]
  DatabaseError(#[from] WitherError),
  #[error("Could not find user with username `{0}`")]
  UserNotFound(String),
  #[error("The password doesn't match")]
  WrongPassword(#[from] PasswordErrors),
}

#[derive(Debug, Clone, Model, Serialize, Deserialize, SimpleObject)]
#[graphql(complex)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
  /// The user ID
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  /// The user's unique username
  pub username: String,
  /// The user's hashed password, hidden in the graphql schema
  #[graphql(skip)]
  pub password: String,
  /// The user's associated account ids, hidden in the graphql schema
  #[graphql(skip)]
  pub account_ids: Vec<ObjectId>,
  // pub accounts: Option<Vec<Account>>,
}

#[ComplexObject]
impl User {
  /// Populates all user's accounts
  /// The user's associated accounts
  pub async fn accounts(&self, ctx: &Context<'_>) -> Vec<Account> {
    Vec::new()
  }
}

/// Method to create a User, hashing the password
pub async fn create_user(db: &Database, username: String, password: &str) -> Result<User, UserErrors> {
  let password = hash_password(password);
  let account_ids = Vec::new();
  let mut user = User {
    id: None,
    username,
    password,
    account_ids,
  };
  user.save(db, None).await?;
  Ok(user)
}

/// Logs in a user
pub async fn login_user(db: &Database, username: &str, password: &str) -> Result<User, UserErrors> {
  match User::find_one(&db, doc! { "username": username }, None).await? {
    Some(user) => {
      verify_password(password, &user.password)?;
      Ok(user)
    }
    None => Err(UserErrors::UserNotFound(String::from(username))),
  }
}
