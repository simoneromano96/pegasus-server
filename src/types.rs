use redis::Client;
use wither::mongodb::Database;

use crate::graphql::User;

/// App context
pub struct AppContext {
  /// The users database
  pub db: Database,
  /// The redis client
  pub redis: Client,
}

#[derive(Debug)]
/// A structure representing the user session
pub struct UserSession {
  /// The user
  pub user: User,
  /// Used for reading, deleting or updating session
  pub session_id: String,
}
