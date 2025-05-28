use std::net::SocketAddr;
use anyhow::Result;


use crate::mqtt;
use futures::future::{try_join_all, TryJoinAll};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use crate::http::HttpServer;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub mqtt_address: String,
    pub http_address: String,
    pub basic_path: String,
    pub http_auth_token: String, // Added http_auth_token field
}

pub struct Server {
    pub handler: TryJoinAll<JoinHandle<Result<()>>>,
    pub mqtt_address: SocketAddr,
    pub http_address: SocketAddr,
    pub basic_path : String,
    pub http_auth_token: String, // Added http_auth_token field
}

impl Server {
    
    pub fn new(config: ServerConfig) -> Result<Self> {
        
        let (acceptor,http_address) = HttpServer::try_bind(config.http_address.parse::<SocketAddr>()?)?;
        let (mqtt_address, mqtt_listener) = mqtt::MqttServer::try_bind(config.mqtt_address.parse::<SocketAddr>()?)?;
        let mqtt_handler = tokio::spawn(async { mqtt::MqttServer::start_rmqtt_server(mqtt_listener).await });
        let http_handler = tokio::spawn(async { HttpServer::start_http_server(acceptor).await });
        // Use the http_auth_token from ServerConfig
        HttpServer::set_http_config(config.basic_path.clone(), config.http_auth_token.clone());
        let handler = try_join_all(vec![mqtt_handler, http_handler]);
        Ok(Server {
            handler,
            mqtt_address,
            http_address,
            basic_path: config.basic_path,
            http_auth_token: config.http_auth_token, // Store http_auth_token
        })
    }
    
    pub fn get_config(&self) -> ServerConfig {
        ServerConfig {
            mqtt_address: self.mqtt_address.to_string(),
            http_address: self.http_address.to_string(),
            basic_path: self.basic_path.clone(),
            http_auth_token: self.http_auth_token.clone(), // Retrieve http_auth_token
        }
    }
}


