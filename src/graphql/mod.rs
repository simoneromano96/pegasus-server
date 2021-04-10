pub mod account;
pub mod user;

pub use account::{Account, EncryptedAccount};
use account::{AccountMutation, AccountQuery};
use async_graphql::{EmptySubscription, MergedObject, Schema};
pub use user::User;
use user::{UserMutation, UserQuery};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, AccountQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, AccountMutation);

pub type MySchema = Schema<Query, Mutation, EmptySubscription>;
