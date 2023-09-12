use std::env;
use std::fs::File;
use std::sync::Arc;
use actix_web::{App, get, HttpServer, post, Responder, web};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log::{info, LevelFilter};

use crate::config::{Config, NodeConfig};
use crate::engine::message::{Message, Receipt};
use crate::engine::start_engine;

mod engine;
mod config;


fn main() {
    init_log();


    let cfg = parse_cfg();
    let mut rt = tokio::runtime::Runtime::new().unwrap();


    let cfg_clone = cfg.clone();
    rt.spawn(async move {
        start_engine(cfg_clone).await
    });
    rt.block_on(async move {

        let curr_node_cfg =  cfg.get_curr_node_cfg();
        HttpServer::new(|| {
            App::new().service(receive_msg)
        }).workers(2)
            .bind((curr_node_cfg.host.as_str(), curr_node_cfg.port)).expect("启动失败！")
            .run()
            .await
    }).expect("HttpServer 报错！");
}
/// 解析配置文件，从环境变量中获取
/// cfg： 配置环境所在的路径
/// node_id： 当前节点的唯一标识
fn parse_cfg() -> Arc<Config> {
    let cfg_path = env::var("cfg").expect("从环境变量中读取cfg失败，请指定").as_str().to_owned();
    info!("cfg_path={:?}",cfg_path);

    let cfg_file = File::open(cfg_path).expect("读取配置文件失败！");
    let mut cfg:Config = serde_yaml::from_reader(cfg_file).expect("读取配置文件失败！");

    if let Ok(node_id) = env::var("node_id") {
        info!("读取到了环境变量配置的node_id:{:?},覆盖更新！",node_id);
        cfg.curr_node_id = node_id.parse::<u16>().expect("解析node_id失败，请检查环境变量的配置！");
    }

    info!("cfg:{:?}", cfg);
    Arc::new(cfg)
}


pub fn init_log() {
    let stdout = ConsoleAppender::builder().build();

    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();

    info!("日志初始化成功！");
}

#[post("/receive_msg")]
async fn receive_msg(msg: web::Json<Message>) -> impl Responder {
    web::Json(Receipt::Invalid(format!("receive_msg {:?}!", msg)))
}