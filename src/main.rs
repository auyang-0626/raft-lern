use std::sync::Arc;
use std::time::Duration;
use actix_web::HttpServer;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log::{info, LevelFilter};
use tokio::time::sleep;

use crate::config::Config;
use crate::engine::start_engine;

mod error;
mod engine;
mod config;

#[tokio::main]
async fn main() {
    init_log();


    let cfg = Config {
        node_id: 1,
        ..Default::default()
    };
    info!("cfg:{:?}", cfg);

    let engine_client = start_engine(Arc::new(cfg)).unwrap();

    HttpServer::new(|| {
        App::new().service(greet)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
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