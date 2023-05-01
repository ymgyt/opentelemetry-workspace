use crate::otel::ContextPropagationExt;
use reqwest::{Method, Url};

pub struct RestClient {
    inner: reqwest::Client,
    base: Url,
}

impl RestClient {
    pub fn new() -> Self {
        Self {
            inner: reqwest::ClientBuilder::new().build().unwrap(),
            base: Url::parse("http://localhost:8001").unwrap(),
        }
    }

    pub async fn foo(&self) -> anyhow::Result<()> {
        self.inner
            .request(Method::GET, self.base.join("foo").unwrap())
            .propagate_otel_ctx()
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
