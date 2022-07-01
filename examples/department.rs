use anyhow::Result;
use dingtalk_rs::{client::DepartmentManager, Client};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let client = Client::new_from_env()?;

    let resp = client.department_list(None).await?;
    info!("department_list resp:{}", serde_json::to_string(&resp)?);

    Ok(())
}
