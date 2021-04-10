use std::string::FromUtf8Error;

use anyhow::Result;
use async_graphql::{ComplexObject, Context, SimpleObject};
use log::debug;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wither::{
  bson::{doc, oid::ObjectId, Binary},
  mongodb::Database,
  prelude::*,
  WitherError,
};

use crate::utils::{CryptoErrors, decrypt_bson_binary, decrypt_data, decrypt_optional_bson_binary, encrypt_bson_binary};
use crate::{graphql::User, types::UserSession};

#[derive(Debug, Error)]
pub enum AccountErrors {
  #[error("{0}")]
  DatabaseError(#[from] WitherError),
  #[error("Could not find account with id `{0}`")]
  AccountNotFound(String),
  #[error("{0}")]
  DecryptionError(#[from] CryptoErrors),
  #[error("Invalid UTF-8 string! {0}")]
  DecodeError(#[from] FromUtf8Error)
}

#[derive(Debug, Clone, Model, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[model()]
/// An encrypted account represents a user's credential, for example `google.com` may have `user@gmail.com`
///
/// This is only the DB representation
pub struct EncryptedAccount {
  /// This encrypted account ID
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  /// Account username, this can be, for example, an email, an actual username etc.
  pub username: Binary,
  /// Related URI, for example google.com
  pub uri: String,
  /// Account password, this is optional, for example some sites require only sms, email, etc. login
  pub password: Option<Binary>,
  /// This account notes, can be any additional data
  pub notes: Option<Binary>,
}

/// GQL model
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct Account {
  /// The account's ID
  pub id: ObjectId,
  /// Account username, this can be, for example, an email, an actual username etc.
  pub username: String,
  /// Related URI, for example google.com
  pub uri: String,
  /// Account password, this is optional, for example some sites require only sms, email, etc. login
  pub password: Option<String>,
  /// This account notes, can be any additional data
  pub notes: Option<String>,
}

/// Creates a new `EncryptedAccount`
pub async fn create_account(
  db: &Database,
  user: &mut User,
  master_password: String,
  username: String,
  uri: String,
  password: Option<String>,
  notes: Option<String>,
) -> Result<Account> {
  // Get and increment user's nonce
  let nonce = user.nonce.bytes.clone();

  debug!("{:?}", &user);
  // Encrypt username
  let username =
    encrypt_bson_binary(master_password.as_bytes(), &nonce, username.as_bytes());

  // Encrypt password
  let mut encrypted_password = None;
  if let Some(password) = password {
    encrypted_password = Some(encrypt_bson_binary(
      master_password.as_bytes(),
      &nonce,
      password.as_bytes(),
    ));
  }

  // Encrypt notes
  let mut encrypted_notes = None;
  if let Some(notes) = notes {
    encrypted_notes = Some(encrypt_bson_binary(
      master_password.as_bytes(),
      &nonce,
      notes.as_bytes(),
    ));
  }

  // Create the new account
  let mut new_account = EncryptedAccount {
    id: None,
    username,
    uri: uri.clone(),
    password: encrypted_password,
    notes: encrypted_notes,
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
  Ok(decrypt_account(
    new_account,
    master_password.as_bytes(),
    &nonce,
    uri,
  )?)
}

/// Gets and decrypts the account
pub async fn get_account(
  db: &Database,
  user: &User,
  account_id: ObjectId,
  master_password: String,
) -> Result<Account, AccountErrors> {
  // Get account from database
  let encrypted_account = EncryptedAccount::find_one(db, doc! { "_id": &account_id }, None).await?;

  // If we find the account
  if let Some(encrypted_account) = encrypted_account {
    debug!("{:?}", &encrypted_account);

    let nonce = &user.nonce.bytes;

    let uri = encrypted_account.uri.clone();

    let account = decrypt_account(encrypted_account, master_password.as_bytes(), nonce, uri)?;

    debug!("{:?}", &account);

    Ok(account)
  } else {
    Err(AccountErrors::AccountNotFound(account_id.to_string()))
  }
}

fn decrypt_account(
  encrypted_account: EncryptedAccount,
  key: &[u8],
  nonce: &[u8],
  uri: String,
) -> Result<Account, AccountErrors> {
  // Get and decrypt username
  let username_bytes = decrypt_bson_binary(key, nonce, encrypted_account.username)?;
  let username = String::from_utf8(username_bytes)?;

  // Get and decrypt password
  let mut password = None;
  if let Some(Ok(bytes)) = decrypt_optional_bson_binary(key, nonce, encrypted_account.password) {
    password = Some(String::from_utf8(bytes)?);
  }

  // Get and decrypt notes
  let mut notes = None;
  if let Some(Ok(bytes)) = decrypt_optional_bson_binary(key, nonce, encrypted_account.notes) {
    notes = Some(String::from_utf8(bytes)?);
  }

  // Create Account
  Ok(Account {
    id: encrypted_account.id.unwrap(),
    username,
    uri,
    password,
    notes,
  })
}
