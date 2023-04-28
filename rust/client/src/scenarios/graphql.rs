use crate::generated::query;
use goose::prelude::{GooseUser, TransactionResult};
use graphql_client::{GraphQLQuery, Response};

pub async fn hello(user: &mut GooseUser) -> TransactionResult {
    let body = query::Hello::build_query(query::hello::Variables {});
    let mut goose = user.post_json("/graphql", &body).await?;

    let Ok(response) = goose.response else {
        panic!("Should success");
    };
    let response: Response<query::hello::ResponseData> = response.json().await.unwrap();
    let tag = "query hello";
    let data = match (response.data, response.errors) {
        (_, Some(errs)) if !errs.is_empty() => {
            let message = errs
                .into_iter()
                .map(|err| format!("{err}"))
                .reduce(|mut s, err| {
                    s.push_str(&err);
                    s
                });

            return user.set_failure(tag, &mut goose.request, None, message.as_deref());
        }
        (Some(data), _) => data,
        _ => panic!("Unexpected response"),
    };

    if data.hello != "HELLO" {
        user.set_failure(
            tag,
            &mut goose.request,
            None,
            Some(&format!("response does not match. got{}", data.hello)),
        )
    } else {
        Ok(())
    }
}
