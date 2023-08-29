use std::sync::Arc;

use log::error;
use tokio::time;

use crate::engine::client::EngineClient;
use crate::engine::Notice;

/// 启动定时器
pub(crate) fn start_tick_loop(client: Arc<EngineClient>, duration: u64) {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_millis(duration));
        loop {
            interval.tick().await;
            if let Err(e) = client.send_notice_event(Notice::Tick).await {
                error!("tick_loop err,{:?}",e);
                break;
            }
        }
    });
}
