use anyhow::Result;
use dingtalk_rs::{client::WorkNotifier, Client};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let agent_id = std::env::var("AGENT_ID")?.parse()?;
    let c = Client::new_from_env()?;

    // let msg_body = dingtalk_rs::client::MessageBody::Text {
    //     content: "这是一个来自Rust版本SDK的文本消息".to_string(),
    // };
    // let params = dingtalk_rs::client::ParamsWorkNotificationSend {
    //     agent_id: agent_id,
    //     msg: msg_body.into(),
    //     userid_list: Some(String::from("025618665127939390,023442065637615370")),
    //     ..Default::default()
    // };
    // let task_id = c.work_notification_send(params).await?;
    // info!("task_id: {}", task_id);

    let res = c
        .work_notification_send_progress_get(agent_id, 636407413173)
        .await?;
    info!(
        "work_notification_send_progress_get: {}",
        serde_json::to_string(&res)?
    );

    let res = c
        .work_notification_send_result_get(agent_id, 636407413173)
        .await?;
    info!(
        "work_notification_send_result_get: {}",
        serde_json::to_string(&res)?
    );
    Ok(())
}
