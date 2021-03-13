use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wither::{bson::doc, bson::oid::ObjectId, mongodb::Database, prelude::*, WitherError};

use crate::utils::{hash_password, verify_password};

#[derive(Debug, Error)]
pub enum UserErrors {
	#[error("{0}")]
	DatabaseError(#[from] WitherError),
	#[error("Wrong password")]
	WrongPassword,
	#[error("Could not find user with username: `{0}`")]
	UserNotFound(String),
}

// Define a model. Simple as deriving a few traits.
#[derive(Debug, Model, Serialize, Deserialize, SimpleObject)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
	/// The ID of the model.
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ObjectId>,
	/// The user's username address.
	pub username: String,
	/// The user's (HASHED) password.
	pub password: String,
}

impl User {
	/// Creates a new user
	pub async fn create(db: &Database, username: String, password: &str) -> Result<Self, UserErrors> {
		let password = hash_password(password);

		let mut user = User {
			id: None,
			username,
			password,
		};

		user.save(db, None).await?;

		Ok(user)
	}

	pub async fn login(db: &Database, username: &str, password: &str) -> Result<Self, UserErrors> {
		let maybe_user = Self::find_one(db, doc! { "username": username }, None).await?;
		if let Some(user) = maybe_user {
			if verify_password(password, &user.password) {
				Ok(user)
			} else {
				Err(UserErrors::WrongPassword)
			}
		} else {
			Err(UserErrors::UserNotFound(username.to_string()))
		}
	}
}
