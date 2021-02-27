use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use wither::bson::{doc, oid::ObjectId};
use wither::prelude::*;

#[derive(Debug, Model, Serialize, Deserialize, SimpleObject)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}
