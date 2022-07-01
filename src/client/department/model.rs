use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeptBaseResponse {
    /// 部门ID
    dept_id: i64,
    /// 部门名称
    name: String,
    /// 父部门ID
    parent_id: i64,
    /// 是否同步创建一个关联此部门的企业群
    create_dept_group: bool,
    /// 部门群已经创建后，有新人加入部门是否会自动加入该群
    auto_add_user: bool,
}
