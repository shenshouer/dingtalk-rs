use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsDepartmentList {
    #[serde(skip_serializing_if = "Option::is_none")]
    dept_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<ParamLanguage>,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Default)]
    struct Test {
        id: String,
        language: ParamLanguage,
    }

    #[test]
    fn test_to_json_string() {
        let item = Test {
            id: "x".to_string(),
            ..Default::default()
        };
        println!("{:?}", serde_json::to_string(&item));
    }
}
