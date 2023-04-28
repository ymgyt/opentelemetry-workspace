pub mod generated;
pub mod scenarios;

use goose::{
    goose::{Scenario, Transaction},
    scenario, transaction, GooseAttack, GooseError,
};
use scenarios::{graphql::hello, healthcheck::health_check};

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("Healthcheck").register_transaction(transaction!(health_check)),
        )
        .register_scenario(scenario!("Hello").register_transaction(transaction!(hello)))
        .execute()
        .await?;

    Ok(())
}
