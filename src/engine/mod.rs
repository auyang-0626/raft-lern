use std::sync::Arc;

use log::info;
use tokio::sync::{mpsc, oneshot};
use tokio::time;

use crate::config::Config;
use crate::engine::client::EngineClient;
use crate::engine::event::Event;
use crate::engine::state::EngineState;
use crate::error::RaftResult;

pub mod event;
mod state;
pub mod client;

/// raft 引擎，接收事件，对外给出指令
pub struct RaftEngine {
    cfg: Arc<Config>,
    state: EngineState,
}

impl RaftEngine {
    pub fn new(cfg: Arc<Config>) -> RaftEngine {
        RaftEngine { cfg, state: EngineState::init() }
    }

    pub async fn handle_notify(&mut self, notify: Event) -> RaftResult<()> {
        info!("接收到:{:?}", notify);
        Ok(())
    }

    pub async fn handle_api(&mut self, req: Event) -> RaftResult<()> {
        info!("接收到:{:?}",req);
        Ok(())
    }
}

/// 启动raft引擎
pub fn start_engine(cfg: Arc<Config>) -> RaftResult<Arc<EngineClient>> {
    let (api_sender, mut api_recv) = mpsc::channel(100000);
    let (notify_sender, mut notify_recv) = mpsc::channel(100000);
    let (shun_down_send, mut shun_down_signal) = oneshot::channel();

    // 事件发送者
    let event_sender = EventSend::new(api_sender, notify_sender, shun_down_send);
    // 定时器
    tick_loop(event_sender.clone(), cfg.heartbeat_interval / 2);

    tokio::spawn(async move {
        let mut engine = RaftEngine::new(cfg);

        loop {
            tokio::select! {
                biased;
                _ = &mut shun_down_signal =>{
                    break;
                }
                Some(notify) = notify_recv.recv() => {

                    engine.handle_notify(notify).await;

                }
                Some(api_res) = api_recv.recv() => {
                    engine.handle_notify(api_res).await;
                }
            }
            ;
        }
        info!("raft engine stopped!");
    });

    Ok(event_sender)
}

/// 启动定时器
pub(crate) fn tick_loop(client: Arc<EngineClient>, duration: u64) {
    tokio::spawn(async move {
        let mut interval = time::interval(time::Duration::from_millis(duration));
        loop {
            interval.tick().await;
            client.send_api_event(Event::Tick).await;
        }
    });
}
