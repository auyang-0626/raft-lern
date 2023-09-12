use std::sync::Arc;

use log::{debug};
use tokio::sync::mpsc::Sender;
use anyhow::Result;

use crate::engine::{ Notice};
use crate::engine::message::Message;

/// 消息发送者
pub struct EngineClient {
    api_sender: Sender<Message>,
    notify_sender: Sender<Notice>,
    shun_down_send: tokio::sync::oneshot::Sender<Notice>,
}

impl EngineClient {
    pub fn new(api_sender: Sender<Message>,
               notify_sender: Sender<Notice>,
               shun_down_send: tokio::sync::oneshot::Sender<Notice>) -> Arc<EngineClient> {
        Arc::new(EngineClient {
            api_sender,
            notify_sender,
            shun_down_send,
        })
    }

    pub async fn send_api_event(&self, e: Message) ->Result<()>{
        debug!("send_api_event {:?}",e);
        self.api_sender.send(e).await?;
        debug!("send_api_event success!");
        Ok(())
    }

    pub async fn send_notice_event(&self, e: Notice) ->Result<()>{
        debug!("send_notice_event {:?}",e);
        self.notify_sender.send(e).await?;
        debug!("send_notice_event success!");
        Ok(())
    }
}