use super::User;
use async_graphql::{Context, Object, Result};
use wither::{prelude::*};
use wither::mongodb::Database;

pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn dummy_user(&self) -> User {
        let user = User {
			id: None,
            username: String::from("Test123"),
            password: String::from("test123"),
        };

        user
    }

	async fn read_users(&self, ctx: &Context<'_>) -> Result<String> {
		let db: &Database = ctx.data()?;
		let users = User::find(&db, None, None).await?;
		Ok(String::from("test"))
	}
}
