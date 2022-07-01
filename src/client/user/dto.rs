use serde::{Deserialize, Serialize};

/// 创建用户参数
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ParamsCreateUser {
    /// 员工唯一标识ID（不可修改），企业内必须唯一。
    /// 长度为1~64个字符，如果不传，将自动生成一个userid。
    #[serde(skip_serializing_if = "Option::is_none")]
    userid: Option<String>,
    /// 员工名称，长度最大80个字符
    name: String,
    /// 手机号码，企业内必须唯一，不可重复。
    /// 如果是国际号码、中国港澳台地区号码，请使用+xx-xxxxxx的格式。
    /// 如果公司注册地址是非中国大陆地区，则在添加大陆地区用户时，手机号要使用+86-xxxxxx格式。
    mobile: String,
    /// 是否号码隐藏：
    /// true：隐藏。隐藏手机号后，手机号在个人资料页隐藏，但仍可对其发DING、发起钉钉免费商务电话。
    /// false：不隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    hide_mobile: Option<bool>,
    /// 分机号，长度最大50个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    telephone: Option<String>,
    /// 员工工号，长度最大为50个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    job_number: Option<String>,
    /// 职位，长度最大为200个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    /// 员工个人邮箱，长度最大50个字符
    /// 说明 员工邮箱是唯一的，企业内不能重复
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    /// 员工的企业邮箱，长度最大100个字符
    /// 说明 需满足以下条件，此字段才生效：员工的企业邮箱已开通。
    #[serde(skip_serializing_if = "Option::is_none")]
    org_email: Option<String>,
    /// 员工的企业邮箱类型。
    /// profession: 标准版
    /// base：基础版
    #[serde(skip_serializing_if = "Option::is_none")]
    org_email_type: Option<String>,
    /// 办公地点，长度最大100个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    work_place: Option<String>,
    /// 备注，长度最大2000个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    remark: Option<String>,
    /// 所属部门id列表，每次调用最多传100个部门ID
    dept_id_list: Vec<i64>,
    /// 员工在对应的部门中的排序
    #[serde(skip_serializing_if = "Option::is_none")]
    dept_order_list: Option<Vec<DeptOrder>>,
    /// 员工在对应的部门中的职位
    #[serde(skip_serializing_if = "Option::is_none")]
    dept_title_list: Option<Vec<DeptTitle>>,
    /// 扩展属性，可以设置多种属性，最大长度2000个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    extension: Option<String>,
    /// 是否开启高管模式，默认值false
    /// 开启后，手机号码对所有员工隐藏。
    /// 普通员工无法对其发DING、发起钉钉商务电话。
    /// 高管之间可以发DING、发起钉钉商务电话
    #[serde(skip_serializing_if = "Option::is_none")]
    senior_mode: Option<bool>,
    /// 入职时间，Unix时间戳，单位毫秒
    #[serde(skip_serializing_if = "Option::is_none")]
    hired_date: Option<i64>,
    /// 直属主管的userId
    #[serde(skip_serializing_if = "Option::is_none")]
    manager_userid: Option<u64>,
    /// 登录邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    login_email: Option<String>,
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
    dept_id: Option<i64>,
    /// 员工在部门中的排序
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeptTitle {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    dept_id: Option<u64>,
    /// 员工在部门中的职位
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}
