use log::error;
use tokio::sync::mpsc::error::SendError;
use crate::engine::{Message, Notice};

pub type RaftResult<T> = std::result::Result<T, RaftError>;

#[derive(Debug)]
pub enum RaftError {
    BootFailed(String),
    RunningError(String),
    Stopped(String),
}

impl RaftError {
    pub fn boot_failed(msg: &str) -> RaftError {
        RaftError::BootFailed(format!("启动失败，原因:{}", msg))
    }
}


impl From<SendError<Notice>> for  RaftError{
    fn from(e: SendError<Notice>) -> Self {
        error!("发送失败:{:?}",e);
        RaftError::RunningError(e.to_string())
    }
}

impl From<SendError<Message>> for  RaftError{
    fn from(e: SendError<Message>) -> Self {
        error!("发送失败:{:?}",e);
        RaftError::RunningError(e.to_string())
    }
}