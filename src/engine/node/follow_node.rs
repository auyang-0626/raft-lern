use std::sync::Arc;
use log::info;
use crate::config::Config;
use crate::engine::node::Node;
use crate::engine::state::term::Term;

#[derive(Debug, Default)]
pub(crate) struct FollowNode {

    pub(crate) cfg:Arc<Config>,
    /// 当前任期
    pub(crate) current_term: Term,
    /// 标记为竞争者
    pub(crate) candidate_flag:bool,
}

impl Node for FollowNode {
    fn handle_tick(&mut self) {
        // 检查超时
        if self.current_term.is_expire() {
            self.start_elect();
        }
    }

}

impl FollowNode {

    pub(crate) fn new(cfg:Arc<Config>)->FollowNode{
        FollowNode{ cfg, current_term: Default::default(),candidate_flag:false }
    }


    /// 开始选举
    fn start_elect(&mut self){
        info!("开始新的选举...");
        // 增大term
        self.current_term = self.current_term.increment(&self.cfg);
        // 成为竞争者
        self.candidate_flag = true;
    }
}