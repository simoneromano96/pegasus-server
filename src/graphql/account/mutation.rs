use async_graphql::{Context, Object, Result};

use super::{create_account, Account};
use crate::types::{AppContext, UserSession};

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
    let UserSession { user, .. } = ctx.data()?;
    let AppContext { db, .. } = ctx.data()?;

    let account = create_account(db, user, user_password, username, password, notes).await?;
    Ok(account)
  }
}
