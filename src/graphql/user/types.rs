use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct User {
    pub username: String,
    pub password: String,
}
