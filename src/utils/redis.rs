use anyhow::Result;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};

/// Initialise redis client
/// Panics if couldn't connect to redis server
pub fn init_redis_client() -> Client {
    Client::open("redis://127.0.0.1/").expect("Could not init redis")
}

/// Helper to set a serializable value into redis
pub async fn redis_serialize_set<T>(data: &T, redis: &Client, key: &String) -> Result<()>
where
    T: ?Sized + Serialize,
{
    let mut redis_connection = redis.get_async_connection().await?;
    let value = serde_json::to_string(data)?;
    redis_connection.set(key, &value).await?;
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
