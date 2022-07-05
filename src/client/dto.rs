use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
// #[serde(untagged)]
pub enum ParamLanguage {
    #[serde(rename = "zh_CN")]
    CN,
    #[serde(rename = "en_US")]
    Us,
}

impl Default for ParamLanguage {
    fn default() -> Self {
        Self::CN
    }
}
