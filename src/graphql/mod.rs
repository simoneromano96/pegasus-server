pub mod user;
pub mod account;

use async_graphql::{EmptySubscription, MergedObject, Schema};
pub use user::User;
pub use account::Account;
use user::{UserMutation, UserQuery};
use account::{AccountQuery};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type MySchema = Schema<Query, Mutation, EmptySubscription>;
