use crate::Result;
use async_trait::async_trait;

/// 用户管理
#[async_trait]
pub trait UserManager {
    // 服务端API
    /// 创建用户
    /// https://open.dingtalk.com/document/orgapp-server/user-information-creation
    async fn user_create(&self, params: ParamsCreateUser) -> Result<ResponseUserCreate>;
    /// 根据手机号查询用户, 根据手机号可以查询在职员工的userId。如果员工离职，无法根据手机号获取用户的userId
    /// https://open.dingtalk.com/document/orgapp-server/query-users-by-phone-number
    async fn user_get_by_mobile(&self, mobile: &str) -> Result<String>;

    // 新版服务端API
}

mod dto;
mod user;
pub use dto::*;
mod model;
pub use model::*;
