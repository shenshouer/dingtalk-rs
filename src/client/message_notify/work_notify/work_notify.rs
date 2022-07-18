use super::{
    AsyncSendProgress, AsyncSendResult, ParamsStatusBarUpdate, ParamsWorkNotificationSend,
    ResponseAsyncSendProgress, ResponseAsyncSendResult, ResponseWorkNotificationSend, WorkNotifier,
};
use crate::{
    client::{ResponseFlatten, BASE_URL},
    Client, Result,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl WorkNotifier for Client {
    async fn work_notification_send(&self, params: ParamsWorkNotificationSend) -> Result<i64> {
        let token = self.access_token().await?;
        let resp = self
            .request::<ResponseFlatten<ResponseWorkNotificationSend>>(
                Method::POST,
                &format!(
                    "{BASE_URL}/topapi/message/corpconversation/asyncsend_v2?access_token={token}"
                ),
                Some(serde_json::to_value(&params)?),
            )
            .await?;

        Ok(resp.result.unwrap().task_id)
    }

    async fn status_bar_update(&self, params: ParamsStatusBarUpdate) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<ResponseFlatten<()>>(
            Method::POST,
            &format!(
                "{BASE_URL}/topapi/message/corpconversation/status_bar/update?access_token={token}"
            ),
            Some(serde_json::to_value(&params)?),
        )
        .await?;

        Ok(())
    }

    async fn work_notification_send_progress_get(
        &self,
        agent_id: i64,
        task_id: i64,
    ) -> Result<AsyncSendProgress> {
        let token = self.access_token().await?;
        let resp = self
            .request::<ResponseFlatten<ResponseAsyncSendProgress>>(
                Method::POST,
                &format!(
                    "{BASE_URL}/topapi/message/corpconversation/getsendprogress?access_token={token}"
                ),
                Some(serde_json::json!({
                    "agent_id": agent_id,
                    "task_id": task_id,
                })),
            )
            .await?;

        Ok(resp.result.unwrap().progress)
    }

    async fn work_notification_send_result_get(
        &self,
        agent_id: i64,
        task_id: i64,
    ) -> Result<AsyncSendResult> {
        let token = self.access_token().await?;
        let resp = self
            .request::<ResponseFlatten<ResponseAsyncSendResult>>(
                Method::POST,
                &format!(
                    "{BASE_URL}/topapi/message/corpconversation/getsendprogress?access_token={token}"
                ),
                Some(serde_json::json!({
                    "agent_id": agent_id,
                    "task_id": task_id,
                })),
            )
            .await?;

        Ok(resp.result.unwrap().send_result)
    }

    async fn work_notification_recall_get(&self, agent_id: i64, task_id: i64) -> Result<()> {
        let token = self.access_token().await?;
        self.request::<ResponseFlatten<()>>(
            Method::POST,
            &format!("{BASE_URL}/topapi/message/corpconversation/recall?access_token={token}"),
            Some(serde_json::json!({
                "agent_id": agent_id,
                "msg_task_id": task_id,
            })),
        )
        .await?;

        Ok(())
    }
}
