use crate::{client::RestClient, prelude::*};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};

pub type ApplicationSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello<'a>(&self, cx: &Context<'a>) -> &'static str {
        let client = cx.data_unchecked::<RestClient>();
        match client.foo().await {
            Ok(_) => debug!("successfully foo"),
            Err(err) => error!("{err}"),
        }
        "HELLO"
    }
}
