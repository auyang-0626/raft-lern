use crate::engine::state::term::Term;

pub mod term;

/// 引擎运行时状态
#[derive(Debug, Default)]
pub(crate) struct EngineState {

    pub(crate) role:NodeRole,
    pub(crate) current_term:Term,
    pub(crate) voted_for:u16,


}

impl EngineState {

    pub(crate) fn init()->EngineState {
        EngineState::default()
    }
}

/// 节点角色
#[derive(Debug, Default)]
pub(crate) enum NodeRole {
    #[default]
    Follow,
    Candidate,
    Leader
}



#[derive(Debug, Default)]
pub(crate) struct TimeState {


}