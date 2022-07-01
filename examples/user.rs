use anyhow::Result;
use dingtalk_rs::{Client, UserManager};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new_from_env()?;
    let resp = client.user_get_by_mobile("18500896191").await?;
    info!("user_get_by_mobile resp:{}", resp);

    Ok(())
}
