use anyhow::Result;
use log::{debug, info};
use reqwest::Client;

use crate::config::NodeConfig;
use crate::engine::message::{Message, Receipt};

pub(crate) struct Connected {
    pub target_node_id: u16,
    pub host: String,
    pub port: u16,

    pub client: Client,
}

impl Connected {
    pub fn new(cfg: &NodeConfig) -> Connected {
        let client = reqwest::Client::new();

        Connected {
            target_node_id: cfg.node_id,
            host: cfg.host.to_string(),
            port: cfg.port,
            client,
        }
    }

    pub async fn send_msg(&self, msg: &Message) -> Result<Receipt> {
        let res = self.client
            .post(format!("http://{}:{}/receive_msg", self.host, self.port))
            .json(msg)
            .send().await?;
        debug!("send_msg response:{:?}",res);

        let json = res.text().await?;
        info!("send_msg received json:{}",json);

        Ok(serde_json::from_str(&json)?)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::config::NodeConfig;
    use crate::engine::message::Message;
    use crate::engine::node::connected::Connected;
    use crate::init_log;

    #[tokio::test]
    pub async fn test_send_msg() {
        init_log();

        let cfg = NodeConfig {
            node_id: 1,
            host: "127.0.0.1".to_string(),
            port: 10001,
        };
        let conn = Connected::new(&cfg);
        conn.send_msg(&Message::AskVote).await.expect("发送消息失败！");
    }
}