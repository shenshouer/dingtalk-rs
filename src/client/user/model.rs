use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseUserCreate {
    userid: String,
    #[serde(rename = "unionId")]
    union_id: String,
}
