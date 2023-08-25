pub mod handle;
pub mod send;

/// raft引擎通过接受事件，给出对应的响应
#[derive(Debug)]
pub enum Event {
    // 定时器
    Tick,

}

