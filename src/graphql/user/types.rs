use async_graphql::{ComplexObject, Context, SimpleObject};
use futures::TryStreamExt;
use log::debug;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wither::{
  bson::{doc, oid::ObjectId, Binary, Document},
  mongodb::Database,
  prelude::*,
  WitherError,
};

use crate::{
  graphql::{Account, EncryptedAccount},
  types::AppContext,
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
#[serde(rename_all = "camelCase")]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
  /// The user ID
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  /// The user's unique username
  pub username: String,
  /// The user's hashed password, hidden in the graphql schema
  #[graphql(skip)]
  pub master_password: String,
  /// The user's associated account ids, hidden in the graphql schema
  #[graphql(skip)]
  pub account_ids: Vec<ObjectId>,
  // pub accounts: Option<Vec<Account>>,
  /// The user's nonce
  #[graphql(skip)]
  pub nonce: Binary,
}

#[ComplexObject]
impl User {
  /// Populates all user's accounts
  /// The user's associated accounts
  pub async fn accounts(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<Account>> {
    debug!("Resolving accounts");
    let AppContext { db, .. } = ctx.data()?;
    let user_id = self.id.as_ref().unwrap();
    let lookup_query = doc! {
      "$lookup": {
        "from": EncryptedAccount::COLLECTION_NAME,
        "localField": "accountIds",
        "foreignField": "_id",
        "as": "accounts",
        // "pipeline": [{
        //   "$match": {
        //     "_id": user_id
        //   }
        // }]
      }
    };
    debug!("{:?}", &lookup_query);

    let accounts: Vec<Document> = User::collection(&db)
      .aggregate(vec![lookup_query], None)
      .await?
      .try_collect()
      .await?;
    debug!("{:?}", accounts);

    Ok(Vec::new())
  }
}

/// Method to create a User, hashing the password
pub async fn create_user(
  db: &Database,
  username: String,
  password: &str,
) -> Result<User, UserErrors> {
  // Hash user password
  let password = hash_password(password);

  // Initialize account_ids to an empty array
  let account_ids = Vec::new();

  // Generate a random generator getting entropy from the OS
  let mut rng = ChaCha20Rng::from_entropy();

  // Generate the nonce
  let random_nonce: [u8; 32] = rng.gen();

  debug!("{:?}", &random_nonce);

  let nonce = Binary {
    subtype: wither::bson::spec::BinarySubtype::Encrypted,
    bytes: random_nonce.to_vec(),
  };

  debug!("{:?}", &nonce);

  // Create the user structure
  let mut user = User {
    id: None,
    username,
    master_password: password,
    account_ids,
    nonce,
  };

  // Save the user into the db
  user.save(db, None).await?;

  // Return the new user
  Ok(user)
}

/// Logs in a user
pub async fn login_user(db: &Database, username: &str, password: &str) -> Result<User, UserErrors> {
  // Find a user with the matching username
  match User::find_one(&db, doc! { "username": username }, None).await? {
    // If found verify the password
    Some(user) => {
      verify_password(password, &user.master_password)?;
      Ok(user)
    }
    // Else Notify that the user does not exist
    None => Err(UserErrors::UserNotFound(String::from(username))),
  }
}
