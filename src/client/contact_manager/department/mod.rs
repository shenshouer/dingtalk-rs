use crate::Result;

use async_trait::async_trait;

#[async_trait]
pub trait DepartmentManager {
    /// 创建部门
    /// https://open.dingtalk.com/document/orgapp-server/create-a-department-v2
    async fn department_create(&self, params: ParamsDepartmentCreate) -> Result<i64>;
    /// 更新部门
    /// https://open.dingtalk.com/document/orgapp-server/update-a-department-v2
    async fn department_update(&self, params: ParamsDepartmentUpdate) -> Result<()>;
    /// 删除部门
    /// https://open.dingtalk.com/document/orgapp-server/delete-a-department-v2
    async fn department_delete(&self, dept_id: i64) -> Result<()>;
    async fn department_detail(&self, params: ParamsDepartmentList) -> Result<DeptDetailResponse>;
    /// 获取部门列表
    /// https://open.dingtalk.com/document/orgapp-server/obtain-the-department-list-v2
    async fn department_list(
        &self,
        params: Option<ParamsDepartmentList>,
    ) -> Result<Vec<DeptBaseResponse>>;
    /// 获取子部门ID列表
    /// https://open.dingtalk.com/document/orgapp-server/obtain-a-sub-department-id-list-v2
    async fn department_list_sub_ids(&self, id: i64) -> Result<Vec<i64>>;
    /// 获取指定部门的所有父部门列表
    /// https://open.dingtalk.com/document/orgapp-server/query-the-list-of-all-parent-departments-of-a-department
    async fn department_list_parent_ids_by_dept_id(&self, dept_id: i64) -> Result<Vec<i64>>;
    /// 获取指定用户的所有父部门列表
    /// https://open.dingtalk.com/document/orgapp-server/queries-the-list-of-all-parent-departments-of-a-user
    async fn department_list_parent_ids_by_userid(
        &self,
        userid: &str,
    ) -> Result<Vec<ParentDeptIdList>>;
}

mod dto;
pub use dto::*;
mod model;
pub use model::*;
mod department;
