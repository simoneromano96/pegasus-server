use async_graphql::{Context, ID, Object, Result};
use futures::TryStreamExt;
use log::debug;
use wither::{bson::oid::ObjectId, prelude::*};

use crate::{types::UserSession, AppContext};

use super::{Account, get_account, get_accounts_by_uri};

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
  // Gets all accounts
  // async fn read_account(&self, ctx: &Context<'_>, account_id: ObjectId, master_password: String) -> Result<Account> {
  //   let AppContext { db, redis } = ctx.data()?;
  //   let UserSession { user, .. } = ctx.data()?;
  // 
  //   let a = get_account(db, user, account_id, master_password).await?;
  // 
  //   Ok(a)
  // }
  async fn read_accounts_by_uri(&self, ctx: &Context<'_>, uri: String, master_password: String) -> Result<Vec<Account>> {
    let AppContext { db, redis } = ctx.data()?;
    let UserSession { user, session_id } = ctx.data()?;

    let r = get_accounts_by_uri(db, user, uri, master_password).await?;

    Ok(r)
  }
}
