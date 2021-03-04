use std::convert::TryInto;

use actix_web::{
    cookie::{Cookie, CookieJar},
    HttpMessage, HttpRequest,
};
use anyhow::Result;
use nanoid::nanoid;
use redis::{AsyncCommands, Client};
use thiserror::Error;
use time::Duration;

use super::{redis_deserialize_get, redis_serialize_set};
use crate::types::UserSession;
use crate::{configuration::APP_CONFIG, graphql::User};

#[derive(Error, Debug)]
pub enum SessionErrors {
    #[error("Error encrypting cookie")]
    CookieEncryption,
}

/// Extracts encrypted user session from a HTTP Request
pub async fn get_session(req: HttpRequest, redis: &Client) -> Option<UserSession> {
    let mut user_session = None;

    if let Some(encrypted_cookie) = req.cookie(&APP_CONFIG.cookie.name) {
        // println!("{:?}", &encrypted_cookie);
        // Create cookie jar and set cookie
        let mut jar = CookieJar::new();
        jar.add_original(encrypted_cookie);
        // Decrypt cookie
        if let Some(cookie) = jar
            .private(&APP_CONFIG.cookie.key.as_ref().unwrap())
            .get(&APP_CONFIG.cookie.name)
        {
            // println!("{:?}", &cookie);
            let session_id = cookie.value();
            // println!("{:?}", cookie.value());
            if let Ok(user) = redis_deserialize_get(&redis, session_id).await {
                // println!("{:?}", user);
                user_session = Some(UserSession {
                    user,
                    session_id: session_id.to_string(),
                });
            }
        }
    }
    user_session
}

/// Create an encrypted user session
/// Returns an encrypted cookie
pub async fn create_session(redis: &Client, user: &User) -> Result<String> {
    // Generate random id
    let session_id = nanoid!();
    // Create a session cookie
    let plain_cookie = create_session_cookie(&session_id);

    // println!("{:?}", &plain_cookie);

    // Set cookie
    let mut jar = CookieJar::new();
    jar.private(&APP_CONFIG.cookie.key.as_ref().unwrap())
        .add(plain_cookie);

    // Save redis session
    redis_serialize_set(
        redis,
        &session_id,
        &user,
        Some(APP_CONFIG.cookie.maxage.try_into()?),
    )
    .await?;

    // Return the cookie
    if let Some(encrypted_cookie) = jar.get(&APP_CONFIG.cookie.name) {
        // println!("{:?}", &encrypted_cookie);
        Ok(encrypted_cookie.to_string())
    } else {
        Err(anyhow::Error::from(SessionErrors::CookieEncryption))
    }
}

/// Create an expired cookie
pub async fn destroy_session(redis: &Client, session_id: &str) -> Result<String> {
    let mut redis_connection = redis.get_async_connection().await?;
    redis_connection.del(session_id).await?;
    Ok(create_expired_session_cookie().to_string())
}

/// Build a session cookie
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

/// Build an expired session cookie
fn create_expired_session_cookie<'a>() -> Cookie<'a> {
    Cookie::build(APP_CONFIG.cookie.name.clone(), "")
        .max_age(Duration::zero())
        .finish()
}
