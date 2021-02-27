use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use wither::prelude::*;
use wither::{
    bson::{doc, oid::ObjectId},
    mongodb::Database,
    WitherError,
};

use crate::utils::hash_password;

#[derive(Debug, Model, Serialize, Deserialize, SimpleObject)]
#[model(index(keys = r#"doc!{"username": 1}"#, options = r#"doc!{"unique": true}"#))]
pub struct User {
    /// The ID of the model.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn create(
        db: &Database,
        username: String,
        password: &str,
    ) -> Result<Self, WitherError> {
        let password = hash_password(password);
        let mut new_user = Self {
            id: None,
            username,
            password,
        };
        new_user.save(db, None).await?;
        Ok(new_user)
    }
}
