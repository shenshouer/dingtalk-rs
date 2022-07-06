use super::{
    AdminResponse, ListUserByDeptResponse, ListUserSimpleResponse, PageResult, ParamsCreateUser,
    ParamsUpdateUser, ParamsUserInactiveGet, ParamsUserList, ResponseUserCreate, UserDetail,
    UserGetByUnionIdResponse, UserManager,
};
use crate::{
    client::{ParamLanguage, Response, BASE_URL},
    Client, Result,
};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[async_trait]
impl UserManager for Client {
    async fn user_create(&self, params: ParamsCreateUser) -> Result<ResponseUserCreate> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseUserCreate>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/user/create?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.result.unwrap())
    }

    async fn user_update(&self, params: ParamsUpdateUser) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/topapi/v2/user/update?access_token={token}"),
            Some(serde_json::to_value(&params)?),
        )
        .await?;
        Ok(())
    }

    async fn user_delete(&self, userid: &str) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<Response<()>>(
            Method::POST,
            &format!("{BASE_URL}/topapi/v2/user/delete?access_token={token}"),
            Some(serde_json::json!({ "userid": userid })),
        )
        .await?;
        Ok(())
    }

    async fn user_get(&self, userid: &str, language: Option<ParamLanguage>) -> Result<UserDetail> {
        let params = if let Some(language) = language {
            Some(serde_json::json!({"userid": userid, "language": language}))
        } else {
            Some(serde_json::json!({ "userid": userid }))
        };
        let token = self.access_token().await?;

        let resp = self
            .request::<Response<UserDetail>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/user/get?access_token={token}"),
                params,
            )
            .await?;

        Ok(resp.result.unwrap())
    }

    async fn user_get_by_mobile(&self, mobile: &str) -> Result<String> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ResponseUserGetByMobile>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/user/getbymobile?access_token={token}"),
                Some(json!({ "mobile": mobile })),
            )
            .await?;

        Ok(resp.result.unwrap().userid)
    }

    async fn user_list_simple_by_dept(
        &self,
        params: ParamsUserList,
    ) -> Result<PageResult<ListUserSimpleResponse>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<PageResult<ListUserSimpleResponse>>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/user/listsimple?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.result.unwrap())
    }

    async fn user_list_ids(&self, dept_id: i64) -> Result<Vec<String>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<ListUserByDeptResponse>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/user/listid?access_token={token}"),
                Some(serde_json::json!({ "dept_id": dept_id })),
            )
            .await?;

        Ok(resp.result.unwrap().userid_list)
    }

    async fn user_list(&self, params: ParamsUserList) -> Result<PageResult<UserDetail>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<PageResult<UserDetail>>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/user/list?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.result.unwrap())
    }

    async fn user_count(&self, only_active: bool) -> Result<usize> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<CountUserResponse>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/user/count?access_token={token}"),
                Some(serde_json::json!({ "only_active": only_active })),
            )
            .await?;

        Ok(resp.result.unwrap().count)
    }

    async fn user_inactive_get(
        &self,
        params: ParamsUserInactiveGet,
    ) -> Result<PageResult<Vec<String>>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<PageResult<Vec<String>>>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/inactive/user/v2/get?access_token={token}"),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.result.unwrap())
    }

    async fn user_get_by_unionid(&self, unionid: &str) -> Result<UserGetByUnionIdResponse> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<UserGetByUnionIdResponse>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/user/getbyunionid?access_token={token}"),
                Some(serde_json::json!({ "unionid": unionid })),
            )
            .await?;

        Ok(resp.result.unwrap())
    }

    async fn user_admin_list(&self) -> Result<Vec<AdminResponse>> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<Vec<AdminResponse>>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/user/listadmin?access_token={token}"),
                None,
            )
            .await?;

        Ok(resp.result.unwrap())
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ResponseUserGetByMobile {
    userid: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct CountUserResponse {
    count: usize,
}
