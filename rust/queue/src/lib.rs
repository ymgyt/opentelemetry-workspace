use deadpool::managed::{PoolConfig, Timeouts};
pub use deadpool_lapin::{lapin::Channel, Config, Runtime};

pub fn config() -> Config {
    Config {
        // %2f means vhost "/"
        url: Some(String::from("amqp://guest:guest@localhost:5672/%2f")),
        pool: Some(PoolConfig {
            max_size: 100,
            timeouts: Timeouts::default(),
        }),
        ..Default::default()
    }
}

pub async fn channel() -> anyhow::Result<Channel> {
    let pool = config().create_pool(Some(Runtime::Tokio1))?;
    let connection = pool.get().await?;
    let channel = connection.create_channel().await?;

    Ok(channel)
}
