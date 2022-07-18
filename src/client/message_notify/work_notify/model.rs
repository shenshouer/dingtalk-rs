use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseWorkNotificationSend {
    /// 创建的异步发送任务ID。
    pub task_id: i64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseAsyncSendProgress {
    /// 返回结果
    pub progress: AsyncSendProgress,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AsyncSendProgress {
    /// 取值0~100，表示处理的百分比
    pub progress_in_percent: i32,
    /// 任务执行状态：
    /// 0：未开始
    /// 1：处理中
    /// 2：处理完毕
    pub status: i32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ResponseAsyncSendResult {
    /// 返回结果
    pub send_result: AsyncSendResult,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AsyncSendResult {
    /// 无效的userid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_user_id_list: Option<Vec<String>>,
    /// 因发送消息过于频繁或超量而被流控过滤后实际未发送的userid。
    /// 未被限流的接收者仍会被成功发送。
    /// 限流规则包括：
    ///     给同一用户发相同内容消息一天仅允许一次
    ///     同一个应用给同一个用户发送消息：
    ///     如果是ISV接入方式，给同一用户发消息一天不得超过100次
    ///     如果是企业接入方式，此上限为500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_user_id_list: Option<Vec<String>>,
    /// 发送失败的userid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_user_id_list: Option<Vec<String>>,
    /// 已读消息的userid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_user_id_list: Option<Vec<String>>,
    /// 未读消息的userid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_user_id_list: Option<Vec<String>>,
    /// 无效的部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_dept_id_list: Option<Vec<i64>>,
    /// 推送被禁止的具体原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbidden_list: Option<Vec<SendForbiddenModel>>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SendForbiddenModel {
    /// 流控code：
    /// 143105表示企业自建应用每日推送给用户的消息超过上限
    /// 143106表示企业自建应用推送给用户的消息重复
    pub code: String,
    /// 流控阀值
    pub count: i64,
    /// 被流控员工的userid
    pub userid: String,
}
