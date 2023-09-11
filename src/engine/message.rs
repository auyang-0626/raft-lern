use serde::{Deserialize, Serialize};

/// api请求
#[derive(Debug,Clone,Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Message {
    AskVote,// 请求投票
}

/// api请求的回执
#[derive(Debug,Clone,Deserialize, Serialize)]
pub enum Receipt {
    // 无效请求
    Invalid(String),
}

