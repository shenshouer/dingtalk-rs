use super::{DepartmentManager, DeptBaseResponse, ParamsDepartmentList};
use crate::{
    client::{Response, BASE_URL},
    Client, Result,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl DepartmentManager for Client {
    async fn department_list(
        &self,
        params: Option<ParamsDepartmentList>,
    ) -> Result<Vec<DeptBaseResponse>> {
        let body = if let Some(params) = params {
            Some(serde_json::to_value(&params)?)
        } else {
            None
        };
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<Vec<DeptBaseResponse>>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/department/listsub?access_token={token}"),
                body,
            )
            .await?;

        Ok(resp.result.unwrap())
    }
}
