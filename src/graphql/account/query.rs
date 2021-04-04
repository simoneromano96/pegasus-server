use async_graphql::{Context, ID, Object, Result};
use futures::TryStreamExt;
use log::debug;
use wither::{bson::oid::ObjectId, prelude::*};

use crate::{types::UserSession, AppContext};

use super::{Account, get_account};

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
  /// Gets all accounts
  async fn read_account(&self, ctx: &Context<'_>, account_id: ObjectId, user_password: String) -> Result<Account> {
    let AppContext { db, redis } = ctx.data()?;
    let UserSession { user, .. } = ctx.data()?;

    let a = get_account(db, user, account_id, user_password).await?;

    Ok(a)
  }
}
