use std::sync::Arc;

use crate::config::Config;
use crate::engine::node::connected::Connected;
use crate::engine::state::term::Term;

/**
节点组成的集群，封装和其它节点通讯的能力，以及维护集群状态
 */
#[derive(Debug)]
pub(crate) struct Cluster {
    /// 配置
    pub(crate) cfg: Arc<Config>,
    /// 到集群其它节点的连接
    pub(crate) conns: Vec<Connected>,
    /// 当前任期
    pub(crate) current_term: Term,
}

impl Cluster {
    pub(crate) fn new(cfg: Arc<Config>, current_term: Term) -> Cluster {
        let conns: Vec<Connected> = cfg.nodes.iter()
            .filter(|item| item.node_id != cfg.curr_node_id)
            .map(|item| Connected::new(item))
            .collect();

        Cluster {
            cfg,
            conns,
            current_term,
        }
    }
}