use crate::client::ParamLanguage;
use serde::{Deserialize, Serialize};

/// 创建用户参数
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ParamsCreateUser {
    /// 员工唯一标识ID（不可修改），企业内必须唯一。
    /// 长度为1~64个字符，如果不传，将自动生成一个userid。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid: Option<String>,
    /// 员工名称，长度最大80个字符
    pub name: String,
    /// 手机号码，企业内必须唯一，不可重复。
    /// 如果是国际号码、中国港澳台地区号码，请使用+xx-xxxxxx的格式。
    /// 如果公司注册地址是非中国大陆地区，则在添加大陆地区用户时，手机号要使用+86-xxxxxx格式。
    pub mobile: String,
    /// 是否号码隐藏：
    /// true：隐藏。隐藏手机号后，手机号在个人资料页隐藏，但仍可对其发DING、发起钉钉免费商务电话。
    /// false：不隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_mobile: Option<bool>,
    /// 分机号，长度最大50个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    /// 员工工号，长度最大为50个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    /// 职位，长度最大为200个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 员工个人邮箱，长度最大50个字符
    /// 说明 员工邮箱是唯一的，企业内不能重复
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工的企业邮箱，长度最大100个字符
    /// 说明 需满足以下条件，此字段才生效：员工的企业邮箱已开通。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_email: Option<String>,
    /// 员工的企业邮箱类型。
    /// profession: 标准版
    /// base：基础版
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_email_type: Option<String>,
    /// 办公地点，长度最大100个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_place: Option<String>,
    /// 备注，长度最大2000个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 所属部门id列表，每次调用最多传100个部门ID
    pub dept_id_list: Vec<i64>,
    /// 员工在对应的部门中的排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_order_list: Option<Vec<DeptOrder>>,
    /// 员工在对应的部门中的职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_title_list: Option<Vec<DeptTitle>>,
    /// 扩展属性，可以设置多种属性，最大长度2000个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// 是否开启高管模式，默认值false
    /// 开启后，手机号码对所有员工隐藏。
    /// 普通员工无法对其发DING、发起钉钉商务电话。
    /// 高管之间可以发DING、发起钉钉商务电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senior_mode: Option<bool>,
    /// 入职时间，Unix时间戳，单位毫秒
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hired_date: Option<i64>,
    /// 直属主管的userId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_userid: Option<u64>,
    /// 登录邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_email: Option<String>,
}

impl ParamsCreateUser {
    /// 简单初始化
    pub fn new(name: String, mobile: String, dept_id_list: Vec<i64>) -> ParamsCreateUser {
        Self {
            name,
            mobile,
            dept_id_list,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeptOrder {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<i64>,
    /// 员工在部门中的排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeptTitle {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id: Option<u64>,
    /// 员工在部门中的职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsUpdateUser {
    /// 员工的userId
    pub userid: String,
    /// 员工名称，长度最大80个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否号码隐藏：
    /// true：隐藏
    /// 隐藏手机号后，手机号在个人资料页隐藏，但仍可对其发DING、发起钉钉免费商务电话。
    /// false：不隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_mobile: Option<bool>,
    /// 分机号，长度最大50个字符。
    /// 说明 分机号是唯一的，企业内不能重复
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    /// 员工工号，长度最大50个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_number: Option<String>,
    /// 直属主管的userId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_userid: Option<String>,
    /// 职位，长度最大200个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 员工邮箱，长度最大50个字符。
    /// 说明 员工邮箱是唯一的，企业内不能重复
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 员工的企业邮箱。
    /// 说明 需满足以下条件，此字段才生效：员工的企业邮箱已开通
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_email: Option<String>,
    /// 办公地点，长度最大100个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_place: Option<String>,
    /// 备注，长度最大2000个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// 所属部门ID列表
    pub dept_id_list: Option<String>,
    /// 员工在对应的部门中的排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_order_list: Option<Vec<DeptOrder>>,
    /// 员工在对应的部门中的职位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_title_list: Option<Vec<DeptTitle>>,
    /// 扩展属性，可以设置多种属性，最大长度2000个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// 是否开启高管模式，默认值false
    /// 开启后，手机号码对所有员工隐藏。
    /// 普通员工无法对其发DING、发起钉钉商务电话。
    /// 高管之间可以发DING、发起钉钉商务电话
    #[serde(skip_serializing_if = "Option::is_none")]
    pub senior_mode: Option<bool>,
    /// 入职时间，Unix时间戳，单位毫秒
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hired_date: Option<i64>,
    /// 通讯录语言，取值。
    /// zh_CN：中文（默认值）。
    /// en_US：英文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<ParamLanguage>,
    /// 强制更新的字段，支持清空指定的字段，多个字段之间使用逗号分隔。目前支持字段: manager_userid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update_fields: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsUserList {
    /// 部门ID，如果是根部门，该参数传1。
    /// 企业内部应用，可调用获取部门列表获取
    /// 钉钉三方企业应用，可调用获取部门列表获取
    pub dept_id: i64,
    /// 分页查询的游标，最开始传0，后续传返回参数中的next_cursor值。
    pub cursor: i64,
    /// 分页长度，最大值100
    pub size: u64,
    /// 部门成员的排序规则：
    /// entry_asc：代表按照进入部门的时间升序。
    /// entry_desc：代表按照进入部门的时间降序。
    /// modify_asc：代表按照部门信息修改时间升序。
    /// modify_desc：代表按照部门信息修改时间降序。
    /// custom：代表用户定义(未定义时按照拼音)排序。
    /// 默认值：custom。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_field: Option<OrderField>,
    /// 是否返回访问受限的员工
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contain_access_limit: Option<bool>,
    /// 通讯录语言，取值。
    /// zh_CN：中文（默认值）。
    /// en_US：英文。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<ParamLanguage>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderField {
    EntryAsc,
    EntryDesc,
    ModifyAsc,
    ModifyDesc,
    Custom,
}

impl Default for OrderField {
    fn default() -> Self {
        OrderField::Custom
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsUserInactiveGet {
    /// 是否活跃：
    /// false：未登录
    /// true：登录
    pub is_active: bool,
    /// 部门ID列表，可调用获取部门列表获取，不传表示查询整个企业
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_ids: Option<Vec<i64>>,
    /// 支持分页查询，与size参数同时设置时才生效，此参数代表偏移量，偏移量从0开始
    pub offset: i64,
    /// 支持分页查询，与offset参数同时设置时才生效，此参数代表分页大小，最大100
    pub size: u32,
    /// 查询日期，日期格式为：yyyyMMdd
    pub query_date: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsEmpLeaveRecordList {
    /// 开始时间。
    /// 格式：YYYY-MM-DDTHH:mm:ssZ（ISO 8601/RFC 3339）
    #[serde(rename = "startTime")]
    pub start_time: String,
    /// 结束时间。
    /// 格式：YYYY-MM-DDTHH:mm:ssZ（ISO 8601/RFC 3339）。
    /// 说明
    /// 如果该参数不传，开始时间距离当前时间不能超过365天。
    /// 如果该参数传参，开始时间和结束时间跨度不能超过365天。
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 分页游标。
    /// 如果是首次查询，该参数传0。
    /// 如果是非首次查询，该参数传上次调用本接口返回的nextToken
    #[serde(rename = "nextToken")]
    pub next_token: String,
    /// 每页最大条目数，最大值50
    #[serde(rename = "maxResults")]
    pub max_results: i64,
}
