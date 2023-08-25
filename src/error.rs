pub type RaftResult<T> = std::result::Result<T, RaftError>;

#[derive(Debug)]
pub enum RaftError {
    BootFailed(String),
    Stopped(String),
}

impl RaftError {
    pub fn boot_failed(msg: &str) -> RaftError {
        RaftError::BootFailed(format!("启动失败，原因:{}", msg))
    }
}