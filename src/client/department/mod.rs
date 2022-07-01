use crate::Result;

use async_trait::async_trait;

#[async_trait]
pub trait DepartmentManager {
    /// 获取部门列表
    /// https://open.dingtalk.com/document/orgapp-server/obtain-the-department-list-v2
    async fn department_list(
        &self,
        params: Option<ParamsDepartmentList>,
    ) -> Result<Vec<DeptBaseResponse>>;
}

mod dto;
pub use dto::*;
mod model;
pub use model::*;
mod department;
