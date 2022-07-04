use super::{
    DepartmentManager, DeptBaseResponse, DeptDetailResponse, ParamsDepartmentCreate,
    ParamsDepartmentList, ParamsDepartmentUpdate, ParentDeptIdList,
};
use crate::{
    client::{Response, BASE_URL},
    Client, Result,
};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[async_trait]
impl DepartmentManager for Client {
    async fn department_create(&self, params: ParamsDepartmentCreate) -> Result<i64> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseCreateDepartment>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/department/create?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.result.unwrap().dept_id)
    }

    async fn department_update(&self, params: ParamsDepartmentUpdate) -> Result<()> {
        if params.is_empty() {
            return Err(crate::error::Error::EmptyFiledsUpdate);
        }

        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/topapi/v2/department/update?access_token={token}"),
            Some(serde_json::to_value(&params)?),
        )
        .await?;

        Ok(())
    }

    async fn department_delete(&self, dept_id: i64) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/topapi/v2/department/delete?access_token={token}"),
            Some(serde_json::json!({ "dept_id": dept_id })),
        )
        .await?;

        Ok(())
    }

    async fn department_detail(&self, params: ParamsDepartmentList) -> Result<DeptDetailResponse> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<DeptDetailResponse>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/department/get?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.result.unwrap())
    }

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

    async fn department_list_sub_ids(&self, id: i64) -> Result<Vec<i64>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseListSubIds>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/department/listsubid?access_token={token}"),
                Some(serde_json::json!({ "dept_id": id })),
            )
            .await?;

        Ok(resp.result.unwrap().dept_id_list)
    }

    async fn department_list_parent_ids_by_dept_id(&self, dept_id: i64) -> Result<Vec<i64>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseListParentIds>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/department/listparentbydept?access_token={token}"),
                Some(serde_json::json!({ "dept_id": dept_id })),
            )
            .await?;

        Ok(resp.result.unwrap().parent_id_list)
    }

    async fn department_list_parent_ids_by_userid(
        &self,
        userid: &str,
    ) -> Result<Vec<ParentDeptIdList>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseListParentIdsByUserid>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/department/listparentbyuser?access_token={token}"),
                Some(serde_json::json!({ "userid": userid })),
            )
            .await?;

        Ok(resp.result.unwrap().parent_list)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ResponseCreateDepartment {
    dept_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ResponseListSubIds {
    dept_id_list: Vec<i64>,
}

/// 通过userid获取父部门id列表响应
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseListParentIdsByUserid {
    pub parent_list: Vec<ParentDeptIdList>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ResponseListParentIds {
    parent_id_list: Vec<i64>,
}
