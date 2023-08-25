use std::sync::Arc;

use log::{debug, info};
use tokio::sync::mpsc::Sender;

use crate::engine::event::Event;

/// 消息发送者
pub struct EventSend {
    api_sender: Sender<Event>,
    notify_sender: Sender<Event>,
    shun_down_send: tokio::sync::oneshot::Sender<Event>,
}

impl EventSend {
    pub fn new(api_sender: Sender<Event>,
               notify_sender: Sender<Event>,
               shun_down_send: tokio::sync::oneshot::Sender<Event>) -> Arc<EventSend> {
        Arc::new(EventSend {
            api_sender,
            notify_sender,
            shun_down_send,
        })
    }

    pub async fn send_event(&self, e: Event) {
        debug!("send_event {:?}",e);
        // let res = self.sender.send(e).await;
    }
}