use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: u32,
}

pub trait Responser: Default {
    fn error_code(&self) -> u64;
    fn error_message(&self) -> String;
    fn request_id(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Response<T> {
    request_id: Option<String>,
    #[serde(rename = "errcode")]
    err_code: u64,
    #[serde(rename = "errmsg")]
    err_msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
}

impl<T> Responser for Response<T>
where
    T: Default,
{
    fn error_code(&self) -> u64 {
        self.err_code
    }

    fn error_message(&self) -> String {
        self.err_msg.clone()
    }

    fn request_id(&self) -> String {
        self.request_id
            .clone()
            .unwrap_or("no request id found".to_string())
    }
}

impl<T> Default for Response<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            request_id: Default::default(),
            err_code: Default::default(),
            err_msg: Default::default(),
            result: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct ResponseFlatten<T> {
    request_id: Option<String>,
    #[serde(rename = "errcode")]
    err_code: u64,
    #[serde(rename = "errmsg")]
    err_msg: String,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
}

impl<T> Responser for ResponseFlatten<T>
where
    T: Default,
{
    fn error_code(&self) -> u64 {
        self.err_code
    }

    fn error_message(&self) -> String {
        self.err_msg.clone()
    }

    fn request_id(&self) -> String {
        self.request_id
            .clone()
            .unwrap_or("no request id found".to_string())
    }
}

impl<T> Default for ResponseFlatten<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            request_id: Default::default(),
            err_code: Default::default(),
            err_msg: Default::default(),
            result: Default::default(),
        }
    }
}
