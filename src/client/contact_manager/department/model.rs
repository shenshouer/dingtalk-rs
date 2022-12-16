use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeptBaseResponse {
    /// 部门ID
    pub dept_id: i64,
    /// 部门名称
    pub name: String,
    /// 父部门ID
    pub parent_id: i64,
    /// 是否同步创建一个关联此部门的企业群
    pub create_dept_group: bool,
    /// 部门群已经创建后，有新人加入部门是否会自动加入该群
    pub auto_add_user: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeptDetailResponse {
    /// 部门ID
    pub dept_id: i64,
    /// 部门名称
    pub name: String,
    /// 父部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    /// 部门标识字段。
    /// 说明 第三方企业应用不返回该参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// 是否同步创建一个关联此部门的企业群
    pub create_dept_group: bool,
    /// 部门群已经创建后，有新人加入部门是否会自动加入该群
    pub auto_add_user: bool,
    /// 是否默认同意加入该部门的申请：
    /// true：表示加入该部门的申请将默认同意
    /// false：表示加入该部门的申请需要有权限的管理员同意
    pub auto_approve_apply: bool,
    /// 部门是否来自关联组织：
    /// true：是
    /// false：不是
    /// 说明 第三方企业应用不返回该参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_union_org: Option<bool>,
    /// 教育部门标签：
    /// campus：校区
    /// period：学段
    /// grade：年级
    /// class：班级
    /// 说明 第三方企业应用不返回该参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// 在父部门中的次序值
    pub order: i64,
    /// 部门群ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_group_chat_id: Option<String>,
    /// 部门群是否包含子部门
    pub group_contain_sub_dept: bool,
    /// 企业群群主userId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_dept_owner: Option<String>,
    /// 部门的主管userd列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_manager_userid_list: Option<Vec<String>>,
    /// 是否限制本部门成员查看通讯录：
    /// true：开启限制。开启后本部门成员只能看到指定部门/人的通讯录
    /// false：不限制
    pub outer_dept: bool,
    /// 配置的部门员工可见部门Id列表
    /// 说明 接口是否返回该字段，取决于企业是否设置限制本部门成员查看通讯录，即接口返回的outer_dept值
    /// 限制本部门成员查看通讯录（即outer_dept为true）：outer_permit_depts表示设置的只能看到指定部门/人的部门Id列表
    /// 说明 例如，企业开启了限制本部门成员查看通讯录，本部门成员设置了只能看到指定的2个部门、2位员工的通讯录，其中测试部门1的部门Id为1，
    /// 测试部门2的部门Id为2。调用本接口，获取到设置的部门outer_permit_depts的值为[1,2]。不返回员工列表，即不返回员工小钉1、员工小钉2的信息
    /// 未限制本部门成员查看通讯录（即outer_dept为false）：调用接口不返回outer_permit_depts字段
    pub outer_permit_depts: Vec<i64>,
    /// 配置的部门员工可见员工userId列表
    pub outer_permit_users: Vec<String>,
    /// 是否开启隐藏本部门：
    /// true：开启隐藏本部门。可以设置隐藏范围，如设置向所有人和部门隐藏，或者允许指定部门/人可见。
    /// false：关闭隐藏本部门，即部门在公司通讯录显示
    pub hide_dept: bool,
    ///隐藏部门的员工userId列表。
    pub user_permits: Vec<String>,
    /// 隐藏部门的部门Id列表
    pub dept_permits: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParentDeptIdList {
    pub parent_dept_id_list: Vec<i64>,
}
