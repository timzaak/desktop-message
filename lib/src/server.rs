use std::net::SocketAddr;
use anyhow::Result;


use crate::mqtt;
use crate::http;
use futures::future::{try_join_all, TryJoinAll};
use tokio::task::JoinHandle;
use crate::http::HttpServer;

pub struct Server {
    pub handler: TryJoinAll<JoinHandle<Result<()>>>,
    pub mqtt_address: SocketAddr,
    pub http_address: SocketAddr,
}

impl Server {
    
    pub fn start() -> Result<Self> {
        let (acceptor,http_address) = HttpServer::try_bind("0.0.0.0:5800".parse::<SocketAddr>()?)?;
        let (mqtt_address, mqtt_listener) = mqtt::MqttServer::try_bind("0.0.0.0:1883".parse::<SocketAddr>()?)?;
        let mqtt_handler = tokio::spawn(async { mqtt::MqttServer::start_rmqtt_server(mqtt_listener).await });
        let http_handler = tokio::spawn(async { HttpServer::start_http_server(acceptor).await });
        HttpServer::set_basic_path("/".to_owned());
        let handler = try_join_all(vec![mqtt_handler, http_handler]);
        Ok(Server {
            handler,
            mqtt_address,
            http_address,
        })
        
    }
}


