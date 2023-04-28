use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};

pub type ApplicationSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &'static str {
        "HELLO"
    }
}
