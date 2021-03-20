use async_graphql::{Context, Object, Result};
use futures::TryStreamExt;
use wither::prelude::*;

use crate::{types::UserSession, AppContext};

#[derive(Default)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
  /// Gets all accounts
  async fn read_accounts(&self, ctx: &Context<'_>) -> Result<String> {
    // let AppContext { db, .. } = ctx.data()?;
    Ok(String::from("hello"))
  }
}
