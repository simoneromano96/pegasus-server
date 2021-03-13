pub use async_graphql::MergedObject;
use async_graphql::{EmptySubscription, Schema};

pub mod user;

pub use user::*;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type MySchema = Schema<Query, Mutation, EmptySubscription>;
