/// 任期
#[derive(Debug, Default)]
pub(crate) struct Term {
    // 当前任期的lead id
    pub(crate) node_id: u16,
    // 任期的编号
    pub(crate) term_id: u64,
    // 最近一次接收到心跳时间
    pub(crate)  last_heartbeat_time: u64,
    // 过期时间
    pub(crate) expire_time:u64,
}

impl Term {

    /**
        是否过期，如果是，则开启下一轮选举
     */
    pub(crate) fn is_expire(&self, curr_time:u64) -> bool {
        self.expire_time < curr_time
    }
}