use anyhow::Result;
use dingtalk_rs::{Client, EventSubscriber};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let c = Client::new_from_env()?;
    let records = c.callback_failed_result_get().await?;
    info!(
        "callback_failed_result_get: {}",
        serde_json::to_string(&records)?
    );

    Ok(())
}
