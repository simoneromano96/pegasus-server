use async_graphql::{Context, Object, Result};
use wither::Model;

use crate::types::AppContext;

use super::types::User;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
	async fn signup(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
		let AppContext { db } = ctx.data()?;

		let mut user = User {
			id: None,
			username,
			password,
		};

		user.save(db, None).await?;

		Ok(user)
	}
}
