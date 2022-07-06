use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseUserCreate {
    pub userid: String,
    #[serde(rename = "unionId")]
    pub union_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserDetail {
    /// 员工的userId
    pub userid: String,
    /// 员工在当前开发者企业账号范围内的唯一标识
    pub unionid: String,
    /// 员工姓名
    pub name: String,
    /// 头像。
    // 说明 员工使用默认头像，不返回该字段，手动设置头像会返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 国际电话区号。
    /// 说明 第三方企业应用不返回该字段；如需获取state_code，可以使用钉钉统一授权套件方式获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    /// 员工的直属主管。
    /// 说明 员工在企业管理后台个人信息面板中，直属主管内有值，才会返回该字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_userid: Option<String>,
    /// 手机号码。
    /// 说明
    /// 企业内部应用，只有应用开通通讯录邮箱等个人信息权限，才会返回该字段。
    /// 第三方企业应用不返回该字段，如需获取mobile，可以使用钉钉统一授权套件方式获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 是否号码隐藏：
    /// true：隐藏
    /// false：不隐藏
    /// 说明 隐藏手机号后，手机号在个人资料页隐藏，但仍可对其发DING、发起钉钉免费商务电话。
    pub hide_mobile: bool,
    /// 分机号。
    /// 说明 第三方企业应用不返回该参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    /// 员工工号
    pub job_number: String,
    /// 职位
    pub title: String,
    /// 员工邮箱。
    /// 说明
    /// 企业内部应用，只有应用开通通讯录邮箱等个人信息权限，才会返回该字段。
    /// 第三方企业应用，不返回该参数；如需获取email，可以使用钉钉统一授权套件方式获取
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 办公地点。
    /// 说明
    /// 员工信息面板中该字段必须有值，才正常返回。如果无值，则不返回该字段。
    /// 企业内部应用，只有应用开通通讯录邮箱等个人信息权限，才会返回该字段。
    /// 第三方企业应用，不返回该参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_place: Option<String>,
    /// 备注。
    /// 说明
    /// 员工信息面板中该字段必须有值，才正常返回。如果无值，则不返回该字段。
    /// 企业内部应用，只有应用开通通讯录邮箱等个人信息权限，才会返回该字段。
    /// 第三方企业应用，不返回该参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 是否为专属帐号：
    /// true：是
    /// false：不是
    pub exclusive_account: bool,
    /// 员工的企业邮箱。
    /// 如果员工的企业邮箱没有开通，返回信息中不包含该数据。
    /// 说明 第三方企业应用不返回该参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_email: Option<String>,
    /// 所属部门id列表
    pub dept_id_list: Vec<i64>,
    /// 员工在对应的部门中的排序
    pub dept_order_list: Option<Vec<super::DeptOrder>>,
    /// 扩展属性，最大长度2000个字符。
    /// 说明
    /// 员工信息面板中添加的拓展字段内有值才返回。
    /// 企业内部应用，只有应用开通通讯录邮箱等个人信息权限，才会返回该字段。
    /// 第三方企业应用，不返回该字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// 入职时间，Unix时间戳，单位毫秒。
    /// 说明
    /// 信息面板中入职时间字段内有值才返回。
    /// 第三方企业应用，不返回该参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hired_date: Option<i64>,
    /// 是否激活了钉钉：
    /// true：已激活
    /// false：未激活
    pub active: bool,
    /// 是否完成了实名认证：
    /// true：已认证
    /// false：未认证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub real_authed: Option<bool>,
    /// 员工的企业邮箱类型。
    /// profession：标准版
    /// base：基础版
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_email_type: Option<String>,
    /// 是否为企业的高管：
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senior: Option<bool>,
    /// 是否为企业的管理员
    pub admin: bool,
    /// 是否为企业的老板
    pub boss: bool,
    /// 员工所在部门信息及是否是领导：
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_in_dept: Option<Vec<DeptLeader>>,
    /// 角色列表
    pub role_list: Option<Vec<Role>>,
    /// 当用户来自于关联组织时的关联信息。
    /// 说明 用户所在企业存在关联关系的企业，返回该字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_emp_ext: Option<UnionEmpExt>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeptLeader {
    /// 员工所在部门的部门ID。
    pub dept_id: i64,
    /// 员工在对应的部门中是否是领导：
    /// true：是
    /// false：不是
    pub leader: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Role {
    /// 角色ID
    pub id: i64,
    /// 角色名称
    pub name: String,
    /// 角色组名称
    pub group_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct UnionEmpExt {
    /// 员工id的userId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    /// 当前用户所属的组织的企业corpId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_id: Option<String>,
    /// 关联映射关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_emp_map_list: Option<Vec<UnionEmpMap>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UnionEmpMap {
    /// 关联分支组织中的员工userId
    pub userid: String,
    /// 关联分支组织的企业corpId。
    pub corp_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ListUserSimpleResponse {
    /// 用户的userid
    pub userid: String,
    /// 用户姓名
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ListUserByDeptResponse {
    pub userid_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserGetByUnionIdResponse {
    /// 联系类型：
    /// 0：企业内部员工
    /// 1：企业外部联系人
    pub contact_type: i64,
    /// 用户的userid
    pub userid: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AdminResponse {
    /// 管理员的userid
    pub userid: String,
    /// 管理员角色：
    /// 1：主管理员
    /// 2：子管理员
    pub sys_level: i32,
}
