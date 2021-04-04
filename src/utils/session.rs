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
///
/// This function works like this:
///
/// 1. Take the HTTP request
/// 2. Extract the cookie
/// 3. Decrypts the cookie (AES)
/// 4. Get the session_id from the cookie
/// 5. Gets the value from redis
/// 6. Returns the UserSession struct
pub async fn get_session(redis: &Client, req: HttpRequest) -> Option<UserSession> {
  let mut user_session = None;
  // Get the encrypted cookie from the request
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
///
/// Returns an encrypted cookie
pub async fn create_session(redis: &Client, user: &User) -> Result<String> {
  // Generate new random id
  let session_id = nanoid!();

  // println!("{:?}", &plain_cookie);
  update_session(redis, &session_id, user).await
}

/// Update and renew the current user session, create if not present
pub async fn update_session(redis: &Client, session_id: &str, user: &User) -> Result<String> {
  // Create a session cookie
  let plain_cookie = create_session_cookie(&session_id);

  // Set cookie in the private cookie jar
  let mut jar = CookieJar::new();
  jar
    .private(&APP_CONFIG.cookie.key.as_ref().unwrap())
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

/// Create an expired cookie and delete session from redis
pub async fn destroy_session(redis: &Client, session_id: &str) -> Result<String> {
  // Get redis connection
  let mut redis_connection = redis.get_async_connection().await?;

  // Delete redis value
  redis_connection.del(session_id).await?;

  // Return expired session
  Ok(create_expired_session_cookie().to_string())
}

/// Build a session cooki with the default values
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

/// Build an expired session cookie `max_age: 0`
fn create_expired_session_cookie<'a>() -> Cookie<'a> {
  Cookie::build(APP_CONFIG.cookie.name.clone(), "")
    .max_age(Duration::zero())
    .finish()
}
