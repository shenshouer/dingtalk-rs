use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait EventSubscriber {
    /// 获取推送失败的事件列表
    /// api： https://open.dingtalk.com/document/orgapp-server/obtain-the-event-list-of-failed-push-messages
    async fn callback_failed_result_get(&self) -> Result<CallbackFailedList>;
}

/// 回调加解密
mod callback_crypto;
pub use callback_crypto::DingTalkCrypto;

mod model;
pub use model::*;

mod event_subscribe;
