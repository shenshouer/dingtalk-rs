/// 企业群消息
mod chat_notify;
pub use chat_notify::*;
/// 普通消息
mod message_notify;
pub use message_notify::*;
/// 工作通知
mod work_notify;
pub use work_notify::*;

/// 消息定义
/// 参考： https://open.dingtalk.com/document/orgapp-server/message-types-and-data-format
mod message;
pub use message::*;
