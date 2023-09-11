use std::sync::Arc;

use crate::config::Config;
use crate::engine::state::current_time::get_current_time;

/// 任期
#[derive(Debug, Default)]
pub(crate) struct Term {
    // 当前任期的lead id
    pub(crate) node_id: u16,
    // 任期的编号
    pub(crate) term_id: u64,
    // 最近一次接收到心跳时间
    pub(crate) last_heartbeat_time: u64,
    // 过期时间
    pub(crate) expire_time: u64,
}

impl Term {
    /**
    是否过期，如果是，则开启下一轮选举
     */
    pub(crate) fn is_expire(&self) -> bool {
        self.expire_time < get_current_time()
    }


    /// 开始选举时，自增
    pub(crate) fn increment(&self, cfg: &Arc<Config>) -> Term {
        Term {
            node_id: cfg.curr_node_id,
            term_id: self.term_id + 1,
            last_heartbeat_time: 0,
            expire_time: get_current_time() + cfg.rand_election_timeout(),
        }
    }
}