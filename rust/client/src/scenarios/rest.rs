use goose::prelude::{GooseUser, TransactionResult};

pub async fn foo(user: &mut GooseUser) -> TransactionResult {
    user.get("/foo").await?;
    Ok(())
}
