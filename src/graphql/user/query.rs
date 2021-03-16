use async_graphql::{Context, Object, Result};
use futures::TryStreamExt;
use wither::prelude::*;

use super::User;
use crate::{types::UserSession, AppContext};

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
    let users: Vec<User> = User::find(&db, None, None).await?.try_collect().await?;

    Ok(users)
  }

  /// Get logged user
  async fn me(&self, ctx: &Context<'_>) -> Result<User> {
    let UserSession { user, .. } = ctx.data()?;
    Ok(user.clone())
  }
}
