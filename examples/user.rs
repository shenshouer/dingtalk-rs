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

    // let admin = c.user_admin_list().await?;
    // info!("user_admin_list: {}", serde_json::to_string(&admin)?);

    // let admin_scope = c.user_admin_scope_get("224203514521476195").await?;
    // info!("admin_scope: {}", serde_json::to_string(&admin_scope)?);

    // 新版本服务器端API调用示例
    // let params = dingtalk_rs::client::ParamsEmpLeaveRecordList {
    //     start_time: String::from("2021-07-10T00:00:00Z"),
    //     end_time: None,
    //     next_token: String::from("0"),
    //     max_results: 50,
    // };
    // let records = c.emp_leave_record_list(params).await?;
    // info!("records: {}", serde_json::to_string(&records)?);

    // let user = c.user_get("025618665127939390", None).await?;
    let user = c.user_get("xueyu18237@ipalfish.com", None).await?;
    info!("records: {}", serde_json::to_string(&user)?);

    Ok(())
}
