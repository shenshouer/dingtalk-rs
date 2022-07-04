use crate::error::{new_api_error, Result};
use http::{do_http, PostParameters};
use reqwest::Method;
use resp::{Responser, Token};
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

const BASE_URL: &str = "https://oapi.dingtalk.com";

#[derive(Debug)]
pub struct Client {
    app_key: String,
    app_secret: String,
}

impl Client {
    pub fn new(app_key: String, app_secret: String) -> Self {
        Self {
            app_key,
            app_secret,
        }
    }

    pub fn new_from_env() -> Result<Self> {
        use std::env;
        Ok(Client::new(env::var("APP_KEY")?, env::var("APP_SECRET")?))
    }

    /// 获取企业内部应用的access_token
    /// https://open.dingtalk.com/document/orgapp-server/obtain-orgapp-token
    pub async fn access_token(&self) -> Result<String> {
        let query_body = json!({
            "appkey": self.app_key,
            "appsecret": self.app_secret,
        });

        let resp = do_http(
            Method::GET,
            &format!("{BASE_URL}/gettoken"),
            None,
            Some(query_body),
            None,
        )
        .await?;

        let data = resp.json::<Token>().await?;

        Ok(data.access_token)
    }

    // http 请求
    async fn request<R: Responser + DeserializeOwned + Default>(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<R> {
        let body = if let Some(data) = body {
            Some(PostParameters::json(data))
        } else {
            None
        };

        let resp = do_http(method, url, None, None, body)
            .await?
            .json::<R>()
            .await?;

        if resp.error_code() != 0 {
            return Err(new_api_error(
                resp.error_code(),
                resp.error_message(),
                resp.request_id(),
            ));
        }
        Ok(resp)

        // 调试使用，验证输出结果
        // let resp = do_http(method, url, None, None, body).await?.text().await?;
        // println!("{resp}");
        // Ok(Response::default().result.unwrap())
    }
}

mod http;
mod resp;
pub(crate) use resp::Response;

mod user;
pub use user::*;

mod department;
pub use department::*;
