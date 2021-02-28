use async_graphql::{Context, Object, Result};
use http::header::SET_COOKIE;
use nanoid::nanoid;
use wither::mongodb::Database;

use super::User;
use crate::configuration::APP_CONFIG;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Registers a new user
    async fn signup(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let db: &Database = ctx.data()?;
        Ok(User::create(db, username, &password).await?)
    }

    /// Logs in a user using cookies
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let db: &Database = ctx.data()?;
        let user = User::login(db, &username, &password).await?;
        let session_id = nanoid!();
        // Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
        let cookie_header = create_cookie_header(session_id);

        ctx.append_http_header(SET_COOKIE, cookie_header);
        Ok(user)
    }
}

fn create_cookie_header(session_id: String) -> String {
    let mut cookie_header: String = format!("{}={}", APP_CONFIG.cookie.name, session_id);
    cookie_header.push_str(&format!("; Max-Age={}", APP_CONFIG.cookie.maxage));
    cookie_header.push_str(&format!("; Domain={}", APP_CONFIG.cookie.domain));
    cookie_header.push_str(&format!("; Path={}", APP_CONFIG.cookie.path));
    cookie_header.push_str(&format!("; SameSite={}", APP_CONFIG.cookie.samesite));
    if APP_CONFIG.cookie.secure {
        cookie_header.push_str("; Secure");
    }
    if APP_CONFIG.cookie.httponly {
        cookie_header.push_str("; HttpOnly");
    }

    cookie_header
}
