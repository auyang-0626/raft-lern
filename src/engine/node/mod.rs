use std::sync::Arc;

use crate::config::Config;
use crate::engine::node::follow_node::FollowNode;

pub mod follow_node;
pub mod cluster;
pub mod connected;
pub mod candidate;

pub(crate) trait Node {
    /// 接收到定时器消息
    fn handle_tick(&mut self);
}

/// 节点角色
#[derive(Debug)]
pub(crate) enum NodeRole {
    Follow(FollowNode),
    Leader,
}

impl NodeRole {
    pub(crate) fn new_follow(cfg: Arc<Config>) -> NodeRole {
        NodeRole::Follow(FollowNode::new(cfg))
    }

    pub(crate) fn handle_tick(&mut self) {
        match self {
            NodeRole::Follow(follow) => { follow.handle_tick() }
            NodeRole::Leader => {}
        }
    }
}
