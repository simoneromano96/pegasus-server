use async_graphql::{Context, Object, Result};
use http::header::SET_COOKIE;

use super::{create_account, Account};
use crate::{
  types::{AppContext, UserSession},
  utils::update_session,
};

#[derive(Default)]
pub struct AccountMutation;

#[Object]
impl AccountMutation {
  /// Create an account with the current user
  async fn create_account(
    &self,
    ctx: &Context<'_>,
    user_password: String,
    username: String,
    password: Option<String>,
    notes: Option<String>,
  ) -> Result<Account> {
    let UserSession { user, session_id } = ctx.data()?;
    let AppContext { db, redis } = ctx.data()?;
    let mut updated_user = user.clone();

    let account = create_account(
      db,
      &mut updated_user,
      user_password,
      username,
      password,
      notes,
    )
    .await?;

    let cookie = update_session(redis, session_id, &updated_user).await?;

    ctx.append_http_header(SET_COOKIE, cookie);
    Ok(account)
  }
}
