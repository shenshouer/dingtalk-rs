use crate::{
    client::{PageResult, ParamLanguage},
    Result,
};
use async_trait::async_trait;

/// 用户管理
#[async_trait]
pub trait UserManager {
    // 服务端API
    /// 创建用户
    /// https://open.dingtalk.com/document/orgapp-server/user-information-creation
    async fn user_create(&self, params: ParamsCreateUser) -> Result<ResponseUserCreate>;
    /// 更新用户
    /// https://open.dingtalk.com/document/orgapp-server/user-information-update
    async fn user_update(&self, params: ParamsUpdateUser) -> Result<()>;
    /// 删除用户
    /// https://open.dingtalk.com/document/orgapp-server/delete-a-user
    async fn user_delete(&self, userid: &str) -> Result<()>;
    /// 查询用户详情
    /// https://open.dingtalk.com/document/orgapp-server/query-user-details
    async fn user_get(&self, userid: &str, language: Option<ParamLanguage>) -> Result<UserDetail>;
    /// 获取部门用户基础信息
    /// https://open.dingtalk.com/document/orgapp-server/queries-the-simple-information-of-a-department-user
    async fn user_list_simple_by_dept(
        &self,
        params: ParamsUserList,
    ) -> Result<PageResult<ListUserSimpleResponse>>;
    /// 获取部门用户userid列表
    /// https://open.dingtalk.com/document/orgapp-server/query-the-list-of-department-userids
    async fn user_list_ids(&self, dept_id: i64) -> Result<Vec<String>>;
    /// 获取部门用户详情
    /// https://open.dingtalk.com/document/orgapp-server/queries-the-complete-information-of-a-department-user
    async fn user_list(&self, params: ParamsUserList) -> Result<PageResult<UserDetail>>;
    /// 获取员工人数
    /// https://open.dingtalk.com/document/orgapp-server/obtain-the-number-of-employees-v2
    /// @params only_active: 是否包含未激活钉钉人数：
    ///                      false：包含未激活钉钉的人员数量
    ///                      true：只包含激活钉钉的人员数量
    async fn user_count(&self, only_active: bool) -> Result<usize>;
    /// 获取未登录钉钉的员工列表
    /// https://open.dingtalk.com/document/orgapp-server/queries-the-inactive-users-or-active-users-under-an-enterprise
    async fn user_inactive_get(
        &self,
        params: ParamsUserInactiveGet,
    ) -> Result<PageResult<Vec<String>>>;
    /// 根据手机号查询用户, 根据手机号可以查询在职员工的userId。如果员工离职，无法根据手机号获取用户的userId
    /// https://open.dingtalk.com/document/orgapp-server/query-users-by-phone-number
    async fn user_get_by_mobile(&self, mobile: &str) -> Result<String>;
    /// 根据unionid获取用户userid
    /// https://open.dingtalk.com/document/orgapp-server/query-a-user-by-the-union-id
    async fn user_get_by_unionid(&self, unionid: &str) -> Result<UserGetByUnionIdResponse>;
    /// 获取管理员列表
    /// https://open.dingtalk.com/document/orgapp-server/query-the-administrator-list
    async fn user_admin_list(&self) -> Result<Vec<AdminResponse>>;

    // 新版服务端API
}

mod dto;
mod user;
pub use dto::*;
mod model;
pub use model::*;
