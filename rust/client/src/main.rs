pub mod generated;
pub mod scenarios;

use goose::{
    goose::{Scenario, Transaction},
    scenario, transaction, GooseAttack, GooseError,
};
use scenarios::{graphql::hello, healthcheck::health_check, rest::foo};

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    GooseAttack::initialize()?
        .register_scenario(
            scenario!("healthcheck").register_transaction(transaction!(health_check)),
        )
        .register_scenario(scenario!("hello").register_transaction(transaction!(hello)))
        .register_scenario(scenario!("foo").register_transaction(
            transaction!(foo)
        ))
        .execute()
        .await?;

    Ok(())
}
