pub mod user;

use async_graphql::MergedObject;
pub use user::User;
use user::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);
