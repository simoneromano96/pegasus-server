use actix_web::http::header::SET_COOKIE;
use async_graphql::{Context, Object, Result};

use crate::types::AppContext;

use super::types::User;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
	/// Signup a new user
	async fn signup(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
		let AppContext { db } = ctx.data()?;

		let new_user = User::create(db, username, &password).await?;

		Ok(new_user)
	}

	/// Login a user
	async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
		let AppContext { db } = ctx.data()?;

		let user = User::login(db, &username, &password).await?;

		ctx.insert_http_header(SET_COOKIE, "");

		Ok(user)
	}
}
