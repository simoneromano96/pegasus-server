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

pub async fn create_account(
  db: &Database,
  user: &User,
  user_password: String,
  username: String,
  password: Option<String>,
  notes: Option<String>,
) -> Result<Account> {
  let encrypted_username = encrypt_data(user_password.as_bytes(), username.as_bytes());
  debug!("{:?}", &encrypted_username);

  let encrypted_bson = Binary {
    bytes: encrypted_username,
    subtype: wither::bson::spec::BinarySubtype::Encrypted,
  };

  let mut new_account = Account {
    id: None,
    username: encrypted_bson,
    password,
    notes,
  };

  new_account.save(db, None).await?;

  let account_id = new_account.id.as_ref().unwrap();

  debug!("{:?}", &new_account);

  let mut db_user = user.clone();
  db_user.account_ids.push(account_id.clone());
  db_user.save(db, None).await?;

  debug!("{:?}", &db_user);

  Ok(new_account)
}
