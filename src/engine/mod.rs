use std::sync::Arc;

use log::info;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};

use crate::config::Config;
use crate::engine::client::EngineClient;
use crate::engine::engine_impl::RaftEngine;
use crate::engine::state::current_time::update_current_time;
use crate::engine::tick::start_tick_loop;
use crate::error::RaftResult;

mod state;
pub mod client;
mod tick;
mod engine_impl;
mod node;
pub mod message;


/// 引擎运行期间的通知
#[derive(Debug,Clone)]
pub enum Notice{
    ///
    Tick,
}


/// 启动raft引擎
pub async  fn start_engine(cfg: Arc<Config>)  {
    let (api_sender, mut api_recv) = mpsc::channel(100000);
    let (notify_sender, mut notify_recv) = mpsc::channel(100000);
    let (shun_down_send, mut shun_down_signal) = oneshot::channel();

    // 事件发送者
    let event_sender = EngineClient::new(api_sender, notify_sender, shun_down_send);
    // 定时器
    start_tick_loop(event_sender.clone(), cfg.heartbeat_interval / 2);


        let mut engine = RaftEngine::new(cfg);
        loop {
            tokio::select! {
                biased;
                _ = &mut shun_down_signal =>{
                    break;
                }
                Some(notify) = notify_recv.recv() => {

                    update_current_time();
                    engine.handle_notify(notify).await;

                }
                Some(api_res) = api_recv.recv() => {
                    update_current_time();
                    engine.handle_api(api_res).await;
                }
            }
            ;
        }
        info!("raft engine stopped!");

}


