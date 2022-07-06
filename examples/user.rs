use anyhow::Result;
use dingtalk_rs::{Client, UserManager};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let c = Client::new_from_env()?;

    // let resp = c.user_get_by_mobile("18612424366").await?;
    // info!("user_get_by_mobile resp: {}", resp);

    // let mut params_create =
    //     dingtalk_rs::client::ParamsCreateUser::new("陈亚东".to_string(), "".to_string(), vec![]);
    // c.user_create(params_create).await?;

    // let user_detail = c.user_get("03634229671222446", None).await?;
    // info!("user_get resp: {}", serde_json::to_string(&user_detail)?);

    // let params = dingtalk_rs::client::ParamsUserList {
    //     dept_id: 1,
    //     cursor: 0,
    //     size: 100,
    //     ..Default::default()
    // };
    // let user_sample_list = c.user_list_simple_by_dept(params).await?;
    // info!(
    //     "user_sample_list: {}",
    //     serde_json::to_string(&user_sample_list)?
    // );

    // let user_ids = c.user_list_ids(1).await?;
    // info!("user_ids: {}", serde_json::to_string(&user_ids)?);

    // let params = dingtalk_rs::client::ParamsUserList {
    //     dept_id: 1,
    //     cursor: 0,
    //     size: 100,
    //     ..Default::default()
    // };
    // let user_list = c.user_list(params).await?;
    // info!("user_ids: {}", serde_json::to_string(&user_list)?);

    // let user_count = c.user_count(true).await?;
    // info!("user_count: {}", user_count);

    // let params = dingtalk_rs::client::ParamsUserInactiveGet {
    //     is_active: true,
    //     dept_ids: None,
    //     offset: 1,
    //     size: 100,
    //     query_date: String::from("20220706"),
    // };
    // let users = c.user_inactive_get(params).await?;
    // info!("user_inactive_get: {}", serde_json::to_string(&users)?);

    // let uid = c.user_get_by_unionid("sIXI7wdxNtbdYtsTCrzMtgiEiE").await?;
    // info!("user_get_by_unionid: {}", serde_json::to_string(&uid)?);

    let admin = c.user_admin_list().await?;
    info!("user_admin_list: {}", serde_json::to_string(&admin)?);

    Ok(())
}
