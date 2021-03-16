use async_graphql::{Context, Object, Result};
use futures_lite::stream::StreamExt;
use wither::{bson::doc, Model};

use crate::types::AppContext;

use super::types::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
	async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
		let AppContext { db } = ctx.data()?;
		let users: Vec<User> = User::find(db, doc! {}, None).await?.try_collect().await?;
		Ok(users)
	}
}
