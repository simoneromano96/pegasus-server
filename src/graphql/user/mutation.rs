use async_graphql::{Context, Object, Result};
use http::header::SET_COOKIE;

use super::{create_user, login_user, User};
use crate::types::{AppContext, UserSession};
use crate::utils::{create_session, destroy_session};

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
  /// Registers a new user
  async fn signup(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
    let AppContext { db, .. } = ctx.data()?;
    Ok(create_user(db, username, &password).await?)
  }

  /// Logs in a user using cookies
  async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
    let AppContext { db, redis } = ctx.data()?;
    let user = login_user(db, &username, &password).await?;
    let cookie = create_session(redis, &user).await?;

    ctx.append_http_header(SET_COOKIE, cookie);
    Ok(user)
  }

  /// Logs out a user
  async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
    let AppContext { redis, .. } = ctx.data()?;
    let UserSession { session_id, .. } = ctx.data()?;
    let cookie = destroy_session(redis, session_id).await?;

    ctx.append_http_header(SET_COOKIE, cookie);
    Ok(true)
  }
}
