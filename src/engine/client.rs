use std::sync::Arc;

use log::{debug, info};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

use crate::engine::event::Event;
use crate::error::RaftResult;

/// 消息发送者
pub struct EngineClient {
    api_sender: Sender<Event>,
    notify_sender: Sender<Event>,
    shun_down_send: tokio::sync::oneshot::Sender<Event>,
}

impl EngineClient {
    pub fn new(api_sender: Sender<Event>,
               notify_sender: Sender<Event>,
               shun_down_send: tokio::sync::oneshot::Sender<Event>) -> Arc<EngineClient> {
        Arc::new(EngineClient {
            api_sender,
            notify_sender,
            shun_down_send,
        })
    }

    pub async fn send_api_event(&self, e: Event) ->RaftResult<()>{
        debug!("send_api_event {:?}",e);
        self.api_sender.send(e).await?;
        debug!("send_api_event success!");
        Ok(())
    }
}