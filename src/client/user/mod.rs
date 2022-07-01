use crate::Result;
use async_trait::async_trait;

/// 用户管理
#[async_trait]
pub trait UserManager {
    // 服务端API
    /// 根据手机号查询用户, 根据手机号可以查询在职员工的userId。如果员工离职，无法根据手机号获取用户的userId
    async fn user_get_by_mobile(&self, mobile: &str) -> Result<String>;
    // 新版服务端API
}

mod user;
