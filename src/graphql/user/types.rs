use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use wither::{bson::doc, bson::oid::ObjectId, prelude::*};

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
