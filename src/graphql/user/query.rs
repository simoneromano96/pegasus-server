use async_graphql::{Context, Object, Result};
use futures::stream::StreamExt;
use wither::prelude::*;

use crate::{AppContext, types::UserSession};

use super::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    // async fn dummy_user(&self) -> User {
    //     let user = User {
    //         id: None,
    //         username: String::from("Test123"),
    //         password: String::from("test123"),
    //     };
    //     user
    // }

    /// Gets all current users
    async fn read_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let AppContext { db, .. } = ctx.data()?;
        let mut users = Vec::new();
        let mut users_cursor = User::find(&db, None, None).await?;

        // users_cursor -> next() -> Some(user or error) or Err(?)
        while let Some(user) = users_cursor.next().await {
            users.push(user.unwrap());
        }

        Ok(users)
    }

    /// Get logged user
    async fn me(&self, ctx: &Context<'_>) -> Result<User> {
        let UserSession { user, .. } = ctx.data()?;
        Ok(user.clone())
    }
}
