use std::fmt::Formatter;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error, Unexpected, Visitor};

use crate::error::{RaftError, RaftResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // 当前节点的唯一标识
    pub curr_node_id: u16,
    pub nodes: Vec<NodeConfig>,
    // 心跳时间,ms
    pub heartbeat_interval: u64,
    // 选举最小超时时间
    pub election_timeout_min: u64,
    // 选举最大超时时间
    pub election_timeout_max: u64,
}

#[derive(Serialize, Debug)]
pub struct NodeConfig {
    pub node_id: u16,
    pub host: String,
    pub port: u16,
}

struct MyStringVisitor;

impl<'de> Visitor<'de> for MyStringVisitor {
    type Value = NodeConfig;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("反序列化失败！")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
        let res: Vec<&str> = v.split(":").collect();
        if res.len() == 3 {
            if let Ok(node_id) = res[0].parse::<u16>() {
                if let Ok(port) = res[2].parse::<u16>() {
                    return Ok(NodeConfig {
                        node_id,
                        host: res[1].to_string(),
                        port,
                    });
                }
            }
        }
        Err(Error::invalid_type(Unexpected::Str(v), &self))
    }

}

impl<'de> Deserialize<'de> for NodeConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(MyStringVisitor {})
    }
}


impl Config {
    pub fn valid(&self) -> RaftResult<()> {
        if self.curr_node_id <= 0 {
            return Err(RaftError::boot_failed("未指定NodeId,或者指定的值非法！"));
        }

        Ok(())
    }

    /// 生成选举超时时间
    pub(crate) fn rand_election_timeout(&self) -> u64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.election_timeout_min..self.election_timeout_max)
    }

    /// 获取当前节点的配置
    pub fn get_curr_node_cfg(&self) -> &NodeConfig {
        self.nodes.iter()
            .find(|c| c.node_id == self.curr_node_id)
            .expect("配置有误，找不到当前节点对应的配置！")
    }
}