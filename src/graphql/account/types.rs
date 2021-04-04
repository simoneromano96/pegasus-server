use anyhow::Result;
use async_graphql::{ComplexObject, SimpleObject};
use log::debug;
use serde::{Deserialize, Serialize};
use wither::{
  bson::{doc, oid::ObjectId, Binary},
  mongodb::Database,
  prelude::*,
};

use crate::graphql::User;
use crate::utils::encrypt_data;

#[derive(Debug, Clone, Model, Serialize, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
#[model()]
#[graphql(complex)]
/// An account represents a user's credential, for example `google.com` may have `user@gmail.com`
pub struct Account {
  /// This account ID
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  /// Account username, this can be, for example, an email, an actual username etc.
  #[graphql(skip)]
  pub username: Binary,
  /// Account password, this is optional, for example some sites require only sms, email, etc. login
  pub password: Option<String>,
  /// This account notes, can be any additional data
  pub notes: Option<String>,
}

#[ComplexObject]
impl Account {
  /// Account username, this can be, for example, an email, an actual username etc.
  pub async fn username(&self) -> Vec<u8> {
    self.username.bytes.clone()
  }
}

/// Creates a new `Account`
pub async fn create_account(
  db: &Database,
  user: &mut User,
  user_password: String,
  username: String,
  password: Option<String>,
  notes: Option<String>,
) -> Result<Account> {
  // Get and increment user's nonce
  let nonce = &user.nonce.bytes;

  debug!("{:?}", &user);

  // Encrypt username
  let encrypted_username = encrypt_data(user_password.as_bytes(), username.as_bytes(), nonce);
  debug!("{:?}", &encrypted_username);

  // Create encrypted BSON
  let encrypted_bson = Binary {
    bytes: encrypted_username,
    subtype: wither::bson::spec::BinarySubtype::Encrypted,
  };

  // Create the new account
  let mut new_account = Account {
    id: None,
    username: encrypted_bson,
    password,
    notes,
  };

  // Save the new account
  new_account.save(db, None).await?;

  // Take the new account ID
  let account_id = new_account.id.as_ref().unwrap();

  debug!("{:?}", &new_account);

  // Update user account ids
  user.account_ids.push(account_id.clone());

  // Save the user
  user.save(db, None).await?;

  debug!("{:?}", &user);

  // Return the new account
  Ok(new_account)
}
