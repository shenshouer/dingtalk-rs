use anyhow::Result;
use dingtalk_rs::{client::DepartmentManager, Client};
use dotenv::dotenv;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let c = Client::new_from_env()?;

    // let mut params_create = dingtalk_rs::client::ParamsDepartmentCreate::default();
    // params_create.name = "运维研发".to_string();
    // params_create.parent_id = 1;
    // let new_dept_id = c.department_create(params_create).await?;
    // info!("department_create new_dept_id:{}", new_dept_id);

    // let mut params_update = dingtalk_rs::client::ParamsDepartmentUpdate::default();
    // params_update.dept_id = 666962089;
    // params_update.name = Some(String::from("运维研发(devops)"));
    // c.department_update(params_update).await?;

    // let resp = c.department_list(None).await?;
    // info!("department_list resp:{}", serde_json::to_string(&resp)?);

    // 删除
    // c.department_delete(666962089).await?;

    // let resp = c.department_list(None).await?;
    // info!("department_list resp:{}", serde_json::to_string(&resp)?);

    // let params_get = dingtalk_rs::client::ParamsDepartmentList {
    //     dept_id: Some(1),
    //     ..Default::default()
    // };
    // let resp = c.department_detail(params_get).await?;
    // info!("department_detail resp:{}", serde_json::to_string(&resp)?);

    // let resp = c.department_list_sub_ids(1).await?;
    // info!(
    //     "department_list_sub_ids resp:{}",
    //     serde_json::to_string(&resp)?
    // );

    // let resp = c.department_list_parent_ids_by_dept_id(1).await?;
    // info!(
    //     "department_list_parent_ids resp:{}",
    //     serde_json::to_string(&resp)?
    // );

    let resp = c
        .department_list_parent_ids_by_userid("manager1892")
        .await?;
    info!(
        "department_list_parent_ids_by_userid resp: {}",
        serde_json::to_string(&resp)?
    );

    Ok(())
}
