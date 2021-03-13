use async_graphql::{futures_util::StreamExt, Context, Object, Result};
use wither::{bson::doc, Model};

use crate::types::AppContext;

use super::types::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
	async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
		let AppContext { db } = ctx.data()?;
		let mut users_cursor = User::find(db, doc! {}, None).await?;
		let mut users = Vec::new();
		while let Some(r) = users_cursor.next().await {
			if let Ok(user) = r {
				users.push(user);
			}
		}
		Ok(users)
	}
}
