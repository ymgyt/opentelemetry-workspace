use goose::prelude::*;
use goose_eggs::{validate_and_load_static_assets, Validate};

pub async fn health_check(user: &mut GooseUser) -> TransactionResult {
    let response = user.get("/health_check").await?;

    let validate = Validate::builder().status(200).text("OK").build();
    validate_and_load_static_assets(user, response, &validate).await?;

    Ok(())
}
