use clap::Parser;
use rand::Rng;

use crate::error::{RaftError, RaftResult};

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(long, default_value_t = 0)]
    pub node_id: u16,
    // 心跳时间,ms
    #[arg(long, default_value_t = 2000)]
    pub heartbeat_interval: u64,
    // 选举最小超时时间
    #[arg(long, default_value_t = 15000)]
    pub election_timeout_min: u64,
    // 选举最大超时时间
    #[arg(long, default_value_t = 20000)]
    pub election_timeout_max: u64,
}

#[derive(Parser, Debug)]
pub struct NodeConfig {
    #[arg(long, default_value_t = 0)]
    pub node_id: u16,
}

impl Default for Config {
    fn default() -> Self {
        <Self as Parser>::parse_from(Vec::<&'static str>::new())
    }
}

impl Config {
    pub fn valid(&self) -> RaftResult<()> {
        if self.node_id <= 0 {
            return Err(RaftError::boot_failed("未指定NodeId,或者指定的值非法！"));
        }

        Ok(())
    }

    /// 生成选举超时时间
    pub(crate) fn rand_election_timeout(&self) -> u64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.election_timeout_min..self.election_timeout_max)
    }
}