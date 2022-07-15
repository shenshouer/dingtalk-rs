use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CallbackFailedList {
    pub failed_list: Vec<CallbackFailed>,
    pub has_more: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CallbackFailed {
    pub call_back_tag: String,
    pub event_time: i64,
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

// 事件列表
// https://open.dingtalk.com/document/orgapp-server/event-list-1

/// 通讯录事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventContact {
    /// 通讯录用户增加
    #[serde(rename = "user_add_org")]
    UserAddOrg,
    /// 通讯录用户更改
    #[serde(rename = "user_modify_org")]
    UserModifyOrg,
    /// 通讯录用户离职
    #[serde(rename = "user_leave_org")]
    UserLeaveOrg,
    /// 加入企业后用户激活
    #[serde(rename = "user_active_org")]
    UserActiveOrg,
    /// 通讯录用户被设为管理员
    #[serde(rename = "org_admin_add")]
    OrgAdminAdd,
    /// 通讯录用户被取消设置管理员
    #[serde(rename = "org_admin_remove")]
    OrgAdminRemove,
    /// 通讯录企业部门创建
    #[serde(rename = "org_dept_create")]
    OrgDeptCreate,
    /// 通讯录企业部门修改。
    #[serde(rename = "org_dept_modify")]
    OrgDeptModify,
    /// 通讯录企业部门删除
    #[serde(rename = "org_dept_remove")]
    OrgDeptRemove,
    /// 企业被解散
    #[serde(rename = "org_remove")]
    OrgRemove,
    /// 企业信息发生变更
    #[serde(rename = "org_change")]
    OrgChange,
    /// 员工角色信息发生变更
    #[serde(rename = "label_user_change")]
    LabelUserChange,
    /// 增加角色或者角色组
    #[serde(rename = "label_conf_add")]
    LabelConfAdd,
    /// 删除角色或者角色组
    #[serde(rename = "label_conf_del")]
    LabelConfDel,
    /// 修改角色或者角色组
    #[serde(rename = "label_conf_modify")]
    LabelConfModify,
}

/// 家校通迅录事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventEdu {
    /// 人员身份新增
    #[serde(rename = "edu_user_insert")]
    EduUserInsert,
    /// 人员身份更新
    #[serde(rename = "edu_user_update")]
    EduUserUpdate,
    /// 人员身份删除
    #[serde(rename = "edu_user_delete")]
    EduUserDelete,
    /// 人员关系新增
    #[serde(rename = "edu_user_relation_insert")]
    EduUserRelationInsert,
    /// 人员关系更新
    #[serde(rename = "edu_user_relation_update")]
    EduUserRelationUpdate,
    /// 人员关系删除
    #[serde(rename = "edu_user_relation_delete")]
    EduUserRelationDelete,
    /// 部门节点新增
    #[serde(rename = "edu_dept_insert")]
    EduDeptInsert,
    /// 部门节点更新
    #[serde(rename = "edu_dept_update")]
    EduDeptUpdate,
    /// 部门节点删除
    #[serde(rename = "edu_dept_delete")]
    EduDeptDelete,
}

/// 审批事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventBpms {
    ///审批任务开始，结束，转交
    #[serde(rename = "bpms_task_change")]
    BpmsTaskChange,
    /// 审批实例开始，结束
    #[serde(rename = "bpms_instance_change")]
    BpmsInstanceChange,
}

/// 群会话事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventChat {
    /// 群会话添加人员
    #[serde(rename = "chat_add_member")]
    ChatAddMember,
    /// 群会话删除人员
    #[serde(rename = "chat_remove_member")]
    ChatRemoveMember,
    /// 群会话用户主动退群
    #[serde(rename = "chat_quit")]
    ChatQuit,
    /// 群会话更换群主
    #[serde(rename = "chat_update_owner")]
    ChatUpdateOwner,
    /// 群会话更换群名称
    #[serde(rename = "chat_update_title")]
    ChatUpdateTitle,
    /// 群会话解散群
    #[serde(rename = "chat_disband")]
    ChatDisband,
}

/// 签到事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventCheckIn {
    /// 用户签到事件
    #[serde(rename = "check_in")]
    CheckIn,
}

/// 考勤事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventAttendance {
    /// 员工打卡事件
    #[serde(rename = "attendance_check_record")]
    AttendanceCheckRecord,
    /// 员工排班变更事件
    #[serde(rename = "attendance_schedule_change")]
    AttendanceScheduleChange,
    /// 员工加班事件
    #[serde(rename = "attendance_overtime_duration")]
    AttendanceOvertimeDuration,
}

/// 会议室事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventMeetingroom {
    /// 会议室预定等事件，预定成功、取消等
    #[serde(rename = "meetingroom_book")]
    MeetingroomBook,
    /// 会议室创建、更新、删除等
    #[serde(rename = "meetingroom_room_info")]
    MeetingroomRoomInfo,
}

/// 智能人事事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventHrm {
    /// 人事档案变动等
    #[serde(rename = "hrm_user_record_change")]
    HrmUserRecordChange,
    /// 人事平台员工异动V2
    #[serde(rename = "hrm_mdm_user_change")]
    HrmMdmUserChange,
}

/// 客户动态事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventCrmCustomer {
    /// CRM客户动态变更
    #[serde(rename = "crm_customer_track")]
    CrmCustomerTrack,
}

/// 日程事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventCalendar {
    /// 用户日程发生变更
    #[serde(rename = "calendar_event_change")]
    CalendarEventChange,
}

/// 服务群事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventServicegroupGroup {
    /// 服务群群消息已读未读
    #[serde(rename = "servicegroup_group_message_read_topic_detail")]
    ServicegroupGroupMessageReadTopicDetail,
}

/// 专属钉钉事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventDingtalk {
    /// 企业员工发送文件的检测事件
    #[serde(rename = "org_file_send_check")]
    OrgFileSendCheck,
}

/// 医疗通讯录事件
#[derive(Debug, Deserialize, Serialize)]
pub enum EventIndustryMedical {
    /// 医疗行业科室医疗组变动事件
    #[serde(rename = "industry_medical_dept_event")]
    IndustryMedicalDeptEvent,
    /// 医疗行业科室医疗组属性变动事件
    #[serde(rename = "industry_medical_dept_prop_event")]
    IndustryMedicalDeptPropEvent,
    /// 医疗行业用户属性变动事件
    #[serde(rename = "industry_medical_user_prop_event")]
    IndustryMedicalUserPropEvent,
    /// 医疗行业用户所在科室医疗组变动事件
    #[serde(rename = "industry_medical_user_dept_event")]
    IndustryMedicalUserDeptEvent,
    /// 医疗通讯录全量同步事件
    #[serde(rename = "industry_medical_full_sync")]
    IndustryMedicalFullSync,
}
