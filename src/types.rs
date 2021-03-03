use std::sync::Arc;

use crate::graphql::User;

pub struct AppContext {
    pub db: wither::mongodb::Database,
    pub redis: Arc<redis::Client>,
}

#[derive(Debug)]
pub struct UserSession {
    pub user: User,
    pub session_id: String,
}
