use std::convert::TryInto;

use actix_web::cookie::Cookie;
use async_graphql::{Context, Object, Result};
use http::header::SET_COOKIE;
use nanoid::nanoid;
use redis::AsyncCommands;
use time::Duration;

use super::User;
use crate::configuration::APP_CONFIG;
use crate::types::{AppContext, UserSession};
use crate::utils::redis_serialize_set;

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Registers a new user
    async fn signup(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let AppContext { db, .. } = ctx.data()?;
        Ok(User::create(db, username, &password).await?)
    }

    /// Logs in a user using cookies
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<User> {
        let AppContext { db, redis } = ctx.data()?;
        let user = User::login(db, &username, &password).await?;
        
        let session_id = nanoid!();
        redis_serialize_set(redis, &session_id, &user, Some(APP_CONFIG.cookie.maxage.try_into()?)).await?;
        let cookie = create_cookie(session_id);

        ctx.append_http_header(SET_COOKIE, cookie);
        Ok(user)
    }

    /// Logs out a user
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let AppContext { redis, .. } = ctx.data()?;
        let UserSession { session_id, .. } = ctx.data()?;
        let mut redis_connection = redis.get_async_connection().await?;
        redis_connection.del(session_id).await?;

        ctx.append_http_header(SET_COOKIE, delete_cookie());
        Ok(true)
    }
}

/// Uses the CookieBuilder to create a cookie
/// Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
fn create_cookie(session_id: String) -> String {
    let cookie: Cookie = Cookie::build(APP_CONFIG.cookie.name.clone(), session_id)
        .max_age(Duration::seconds(APP_CONFIG.cookie.maxage.into()))
        .domain(APP_CONFIG.cookie.domain.clone())
        .path(APP_CONFIG.cookie.path.clone())
        .same_site(APP_CONFIG.cookie.samesite.into())
        .secure(APP_CONFIG.cookie.secure)
        .http_only(APP_CONFIG.cookie.httponly)
        .finish();

    cookie.to_string()
}

/// Uses the CookieBuilder to create a cookie that cleans the session
/// Reference: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Set-Cookie
fn delete_cookie() -> String {
    let cookie: Cookie = Cookie::build(APP_CONFIG.cookie.name.clone(), "")
        .max_age(Duration::seconds(0))
        .finish();

    cookie.to_string()
}
