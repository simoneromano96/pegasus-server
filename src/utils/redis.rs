use anyhow::Result;
use redis::{AsyncCommands, Client, RedisError};
use serde::{Deserialize, Serialize};

use crate::configuration::APP_CONFIG;

/// Initialise redis client
/// Panics if couldn't connect to redis server
pub fn init_redis_client() -> Result<Client, RedisError> {
  Client::open(APP_CONFIG.redis.uri.clone())
}

/// Helper to set a serializable value into redis with an optional expire time
pub async fn redis_serialize_set<T>(
  redis: &Client,
  key: &str,
  data: &T,
  expiry: Option<usize>,
) -> Result<()>
where
  T: ?Sized + Serialize,
{
  let mut redis_connection = redis.get_async_connection().await?;
  let value = serde_json::to_string(data)?;
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
  let mut redis_connection = redis.get_async_connection().await?;
  let value: String = redis_connection.get(key).await?;
  let data: T = serde_json::from_str(&value)?;
  Ok(data)
}
