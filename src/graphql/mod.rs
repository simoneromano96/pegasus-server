pub mod user;

use async_graphql::{EmptySubscription, MergedObject, Schema};
pub use user::User;
use user::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type MySchema = Schema<Query, Mutation, EmptySubscription>;
