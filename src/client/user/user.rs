use super::UserManager;
use crate::{
    client::{Response, BASE_URL},
    Client, Result,
};
use async_trait::async_trait;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[async_trait]
impl UserManager for Client {
    async fn user_get_by_mobile(&self, mobile: &str) -> Result<String> {
        let token = self.access_token().await?;
        let resp = self
            .request::<Response<UserGetByMobileResponse>>(
                Method::POST,
                &format!("{BASE_URL}/topapi/v2/user/getbymobile?access_token={token}"),
                Some(json!({ "mobile": mobile })),
            )
            .await?;

        Ok(resp.result.unwrap().userid)
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct UserGetByMobileResponse {
    userid: String,
}
