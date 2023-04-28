use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::Extension;

use crate::schema::ApplicationSchema;

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn graphql(schema: Extension<ApplicationSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
