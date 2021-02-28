use async_graphql::{Context, Object, Result};
use wither::mongodb::Database;

use super::User;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Registers a new user
    async fn signup(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let db: &Database = ctx.data()?;
        Ok(User::create(db, username, &password).await?)
    }

    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let db: &Database = ctx.data()?;
        let user = User::login(db, &username, &password).await?;
        Ok(user)
    }
}
