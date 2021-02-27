use super::User;
use async_graphql::Object;

pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn dummy_user(&self) -> User {
        let user = User {
            username: String::from("Test123"),
            password: String::from("test123"),
        };

        user
    }
}
