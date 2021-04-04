use anyhow::Result;
use redis::{AsyncCommands, Client, RedisError};
use serde::{Deserialize, Serialize};

use crate::configuration::APP_CONFIG;

/// Initialise redis client
///
/// This DOES not actually connect to the redis server, but checks only if the URI is valid
pub fn init_redis_client() -> Result<Client, RedisError> {
  let uri = APP_CONFIG.redis.uri.clone();
  Client::open(uri)
}

/// Helper to set a serializable value into redis with an optional expire time
///
/// the expiry time is in `seconds`
pub async fn redis_serialize_set<T>(
  redis: &Client,
  key: &str,
  data: &T,
  expiry: Option<usize>,
) -> Result<()>
where
  T: ?Sized + Serialize,
{
  // Get connection
  let mut redis_connection = redis.get_async_connection().await?;
  // Serialize data
  let value = serde_json::to_string(data)?;
  // Set data with expire or not
  if let Some(ttl) = expiry {
    redis_connection.set_ex(key, &value, ttl).await?;
  } else {
    redis_connection.set(key, &value).await?;
  }
  Ok(())
}

/// Helper to get a deserializable value from redis
pub async fn redis_deserialize_get<T>(redis: &Client, key: &str) -> Result<T>
where
  T: for<'de> Deserialize<'de>,
{
  // Get connection
  let mut redis_connection = redis.get_async_connection().await?;
  // Get serialized value
  let value: String = redis_connection.get(key).await?;
  // Deserialize data
  let data: T = serde_json::from_str(&value)?;
  Ok(data)
}
