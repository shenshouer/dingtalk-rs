use super::{
    ListUserSimpleResponse, PageResult, ParamsCreateUser, ParamsUpdateUser,
    ParamsUserListSimpeByDept, ResponseUserCreate, UserDetail, UserManager,
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
        params: ParamsUserListSimpeByDept,
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
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct ResponseUserGetByMobile {
    userid: String,
}
