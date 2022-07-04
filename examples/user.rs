use anyhow::Result;
use dingtalk_rs::{client, Client, UserManager};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let c = Client::new_from_env()?;

    let resp = c.user_get_by_mobile("18612424366").await?;
    info!("user_get_by_mobile resp: {}", resp);

    // let mut params_create =
    //     client::ParamsCreateUser::new("陈亚东".to_string(), "".to_string(), vec![]);
    // c.user_create(params_create).await?;

    Ok(())
}
