use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait WorkNotifier {
    /// 发送工作通知
    /// https://open.dingtalk.com/document/orgapp-server/asynchronous-sending-of-enterprise-session-messages
    /// @return task_id
    async fn work_notification_send(&self, params: ParamsWorkNotificationSend) -> Result<i64>;
    /// 更新工作通知状态栏
    /// https://open.dingtalk.com/document/orgapp-server/update-work-notification-status-bar
    async fn status_bar_update(&self, params: ParamsStatusBarUpdate) -> Result<()>;
    /// 获取工作通知消息的发送进度
    /// https://open.dingtalk.com/document/orgapp-server/obtain-the-sending-progress-of-asynchronous-sending-of-enterprise-session
    /// @params
    ///     agent_id: 发送消息时使用的微应用的ID。
    ///               企业内部应用可在开发者后台的应用详情页面查看。
    ///               第三方企业应用可调用获取企业授权信息接口获取。
    ///     task_id:  发送消息时钉钉返回的任务ID。
    ////              说明 仅支持查询24小时内的任务
    async fn work_notification_send_progress_get(
        &self,
        agent_id: i64,
        task_id: i64,
    ) -> Result<AsyncSendProgress>;
    /// 获取工作通知消息的发送结果
    /// https://open.dingtalk.com/document/orgapp-server/gets-the-result-of-sending-messages-asynchronously-to-the-enterprise
    /// @params
    ///     agent_id: 发送消息时使用的微应用的ID。
    ///               企业内部应用可在开发者后台的应用详情页面查看。
    ///               第三方企业应用可调用获取企业授权信息接口获取。
    ///     task_id:  发送消息时钉钉返回的任务ID。
    ////              说明 仅支持查询24小时内的任务
    async fn work_notification_send_result_get(
        &self,
        agent_id: i64,
        task_id: i64,
    ) -> Result<AsyncSendResult>;
    /// 获取工作通知消息的发送结果
    /// https://open.dingtalk.com/document/orgapp-server/gets-the-result-of-sending-messages-asynchronously-to-the-enterprise
    /// @params
    ///     agent_id: 发送消息时使用的微应用的ID。
    ///               企业内部应用可在开发者后台的应用详情页面查看。
    ///               第三方企业应用可调用获取企业授权信息接口获取。
    ///     task_id:  发送消息时钉钉返回的任务ID。
    ////              说明 仅支持查询24小时内的任务
    async fn work_notification_recall_get(&self, agent_id: i64, task_id: i64) -> Result<()>;
}

mod dto;
pub use dto::*;
mod model;
pub use model::*;

mod work_notify;
