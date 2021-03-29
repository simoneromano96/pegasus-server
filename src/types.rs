use redis::Client;
use wither::mongodb::Database;

use crate::graphql::User;

pub struct AppContext {
  pub db: Database,
  pub redis: Client,
}

#[derive(Debug)]
pub struct UserSession {
  pub user: User,
  pub session_id: String,
}
