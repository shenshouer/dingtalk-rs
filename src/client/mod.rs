use crate::error::{new_api_error, Result};
use http::{do_http, PostParameters};
use reqwest::Method;
use resp::{Responser, Token};
use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json::{json, Value};

const BASE_URL: &str = "https://oapi.dingtalk.com";
const BASE_URL_NEW_VERSION: &str = "https://api.dingtalk.com";

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
    async fn request<R: Responser + DeserializeOwned + Serialize + Default>(
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

        // println!("{}", serde_json::to_string(&resp)?);
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

    /// 新版服务端API通用请求
    async fn request_new_version<R: DeserializeOwned + Default>(
        &self,
        method: Method,
        url: &str,
        body: Option<Value>,
    ) -> Result<R> {
        let mut query = None;
        let mut data = None;

        match method {
            Method::GET => query = body,
            Method::POST => {
                data = if let Some(data) = body {
                    Some(PostParameters::json(data))
                } else {
                    None
                };
            }
            _ => {}
        }

        let token = self.access_token().await?;

        let headers = Some(std::collections::HashMap::from([(
            reqwest::header::HeaderName::from_static("x-acs-dingtalk-access-token"),
            token,
        )]));

        let resp = do_http(method, url, headers, query, data)
            .await?
            .json::<R>()
            .await?;

        Ok(resp)

        // 调试使用，验证输出结果
        // let resp = do_http(method, url, None, None, body).await?.text().await?;
        // println!("{resp}");
        // Ok(Response::default().result.unwrap())
    }
}

mod http;
mod resp;
pub(crate) use resp::{Response, ResponseFlatten};

/// 通讯录管理
mod contact_manager;
pub use contact_manager::*;

/// 公共参数
mod dto;
pub use dto::*;
/// 公共模型数据
mod model;
pub use model::*;
/// 事件订阅
mod event_subscribe;
pub use event_subscribe::*;

/// 消息通知
mod message_notify;
pub use message_notify::*;
