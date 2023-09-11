use std::sync::Arc;
use log::{debug, info};
use crate::config::Config;
use crate::engine::node::NodeRole;
use crate::engine::{ Notice};
use crate::engine::message::Message;
use crate::error::RaftResult;


/// raft 引擎，接收事件，对外给出指令
pub struct RaftEngine {
    cfg: Arc<Config>,
    node:NodeRole,
}

impl RaftEngine {
    pub fn new(cfg: Arc<Config>) -> RaftEngine {
        RaftEngine { cfg:cfg.clone(), node: NodeRole::new_follow(cfg)}
    }

    pub async fn handle_notify(&mut self, notify: Notice) -> RaftResult<()> {
        debug!("handle_notify,接收到:{:?}", notify);
        match notify { Notice::Tick => self.handle_tick() }
        Ok(())
    }

    pub async fn handle_api(&mut self, req: Message) -> RaftResult<()> {
        info!("handle_api,接收到:{:?}",req);
        Ok(())
    }

    /// 接收到定时器消息
    fn handle_tick(&mut self) {
        self.node.handle_tick();
    }
}