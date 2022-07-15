use super::{CallbackFailedList, EventSubscriber};
use crate::{
    client::{ResponseFlatten, BASE_URL},
    Client, Result,
};
use async_trait::async_trait;
use reqwest::Method;

#[async_trait]
impl EventSubscriber for Client {
    async fn callback_failed_result_get(&self) -> Result<CallbackFailedList> {
        let token = self.access_token().await?;
        let resp = self
            .request::<ResponseFlatten<CallbackFailedList>>(
                Method::GET,
                &format!("{BASE_URL}/call_back/get_call_back_failed_result?access_token={token}"),
                None,
            )
            .await?;

        Ok(resp.result.unwrap())
    }
}
