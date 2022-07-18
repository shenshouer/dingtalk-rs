use crate::client::message_notify::Message;
use serde::{Deserialize, Serialize};

/// 发送工作通知参数
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsWorkNotificationSend {
    /// 发送消息时使用的微应用的AgentID。
    /// 企业内部应用可在开发者后台的应用详情页面查看。
    /// 第三方企业应用可调用获取企业授权信息接口获取。
    pub agent_id: i64,
    /// 接收者的userid列表，最大用户列表长度100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userid_list: Option<String>,
    /// 接收者的部门id列表，最大列表长度20。
    /// 接收者是部门ID时，包括子部门下的所有用户
    /// 123,345
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_id_list: Option<String>,
    /// 是否发送给企业全部用户。
    /// 说明 当设置为false时必须指定userid_list或dept_id_list其中一个参数的值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_all_user: Option<bool>,
    /// 消息内容，最长不超过2048个字节，支持以下消息类型：
    /// 文本消息
    /// 图片消息
    /// 语音消息
    /// 文件消息
    /// 链接消息
    /// OA消息
    /// 注意 OA消息支持通过status_bar参数设置消息的状态文案和颜色，消息发送后可调用更新工作通知状态栏接口更新消息状态和颜色。
    /// Markdown消息
    /// 卡片消息
    pub msg: Message,
}

/// 更新工作通知状态栏参数
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsStatusBarUpdate {
    /// 发送消息时使用的微应用的AgentID。
    /// 企业内部应用可在开发者后台的应用详情页面查看。
    /// 第三方企业应用可调用获取企业授权信息接口获取。
    pub agent_id: i64,
    /// 工作通知任务ID，
    /// 企业内部应用调用发送工作通知接口获取。
    /// 钉钉三方企业应用调用发送工作通知接口获取。
    pub task_id: i64,
    /// 状态栏值
    pub status_value: String,
    /// 状态栏背景色，推荐0xFF加六位颜色值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_bg: Option<String>,
}
