use reqwest::Client;
use crate::config::NodeConfig;
use crate::engine::message::{Message, Receipt};
use crate::error::RaftResult;

pub(crate) struct Connected {

    pub target_node_id:u16,
    pub host:String,
    pub port:u16,

    pub client:Client,
}

impl Connected {

    pub fn new(cfg:&NodeConfig)->Connected{
        let client = reqwest::Client::new();

        Connected{
            target_node_id: cfg.node_id,
            host: cfg.host.to_string(),
            port: cfg.port,
            client,
        }
    }

    pub async fn send_msg(&self, msg:&Message)->RaftResult<Receipt>{
       let res =  self.client.post(format!("http://{}:{}/receive_msg",self.host,self.port))
            .body(serde_json::to_string(msg)?).send().await?;
       let json = res.text().await?;
       Ok(serde_json::from_str(&json)?)
    }
}

#[cfg(test)]
pub mod tests {
    use log::info;
    use crate::config::NodeConfig;
    use crate::engine::message::Message;
    use crate::engine::node::connected::Connected;
    use crate::init_log;

    #[tokio::test]
    pub async fn test_send_msg(){
        init_log();

        let cfg = NodeConfig{
            node_id: 1,
            host: "127.0.0.1".to_string(),
            port: 10001,
        };
        let conn = Connected::new(&cfg);
        let res = conn.send_msg(&Message::AskVote).await;
        info!("res:{:?}",res);
    }
}