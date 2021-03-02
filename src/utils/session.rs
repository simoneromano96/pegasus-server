use std::convert::TryInto;

use actix_web::{
    cookie::{Cookie, CookieJar},
    HttpMessage, HttpRequest,
};
use anyhow::Result;
use nanoid::nanoid;
use redis::{AsyncCommands, Client};
use time::Duration;

use super::{redis_deserialize_get, redis_serialize_set};
use crate::types::UserSession;
use crate::{configuration::APP_CONFIG, graphql::User};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SessionErrors {
    #[error("Error encrypting cookie")]
    CookieEncryption,
}

/// Extracts encrypted user session from a HTTP Request
pub async fn get_session(req: HttpRequest, redis: &Client) -> Option<UserSession> {
    if let Some(cookie) = req.cookie(&APP_CONFIG.cookie.name) {
        let mut jar = CookieJar::new();
        jar.add_original(cookie);

        if let Some(cookie) = jar
            .private(&APP_CONFIG.cookie.key.as_ref().unwrap())
            .get(&APP_CONFIG.cookie.name)
        {
            let session_id = cookie.value();
            if let Ok(user) = redis_deserialize_get(&redis, session_id).await {
                Some(UserSession {
                    user,
                    session_id: session_id.to_string(),
                });
            }
        }
    }
    None
}

/// Sets an encrypted user session
/// Returns an encrypted cookie
pub async fn create_session(redis: &Client, user: &User) -> Result<String> {
    let session_id = nanoid!();
    let mut jar = CookieJar::new();

    let mut private = jar.private(&APP_CONFIG.cookie.key.as_ref().unwrap());
    let plain_cookie = create_session_cookie(&session_id);
    private.add_original(plain_cookie);

    redis_serialize_set(
        redis,
        &session_id,
        &user,
        Some(APP_CONFIG.cookie.maxage.try_into()?),
    )
    .await?;

    if let Some(encrypted_cookie) = private.get(&APP_CONFIG.cookie.name) {
        Ok(encrypted_cookie.to_string())
    } else {
        Err(anyhow::Error::from(SessionErrors::CookieEncryption))
    }
}

pub async fn destroy_session(redis: &Client, session_id: &str) -> Result<String> {
	let mut redis_connection = redis.get_async_connection().await?;
	redis_connection.del(session_id).await?;
	Ok(create_expired_session_cookie().to_string())
}

/// Builds a session cookie
fn create_session_cookie<'a>(value: &str) -> Cookie<'a> {
    Cookie::build(APP_CONFIG.cookie.name.clone(), value.to_owned())
        .max_age(Duration::seconds(APP_CONFIG.cookie.maxage.into()))
        .domain(APP_CONFIG.cookie.domain.clone())
        .path(APP_CONFIG.cookie.path.clone())
        .same_site(APP_CONFIG.cookie.samesite.into())
        .secure(APP_CONFIG.cookie.secure)
        .http_only(APP_CONFIG.cookie.httponly)
        .finish()
}

/// Builds an expired session cookie
fn create_expired_session_cookie<'a>() -> Cookie<'a> {
	Cookie::build(APP_CONFIG.cookie.name.clone(), "")
        .max_age(Duration::seconds(0))
        .finish()
}
