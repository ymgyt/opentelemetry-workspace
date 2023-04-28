#![allow(clippy::all, warnings)]
pub struct Hello;
pub mod hello {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "Hello";
    pub const QUERY: &str = "query Hello{\n  hello\n}";
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize)]
    pub struct Variables;
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub hello: String,
    }
}
impl graphql_client::GraphQLQuery for Hello {
    type Variables = hello::Variables;
    type ResponseData = hello::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: hello::QUERY,
            operation_name: hello::OPERATION_NAME,
        }
    }
}
