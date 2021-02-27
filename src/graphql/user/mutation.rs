use async_graphql::{Context, Object, Result};
use wither::mongodb::Database;

use super::User;

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn signup(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let db: &Database = ctx.data()?;
        Ok(User::create(db, username, &password).await?)
    }

    // async fn login(&self, username: String, password: String) -> Result<String> {
    //     // User login (generate token)
    //
    // }
}
