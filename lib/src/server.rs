use anyhow::Result;


use crate::mqtt;
use crate::http;
use futures::future::{try_join_all, TryJoinAll};
use tokio::task::JoinHandle;

pub struct Server {
    pub handler: TryJoinAll<JoinHandle<Result<()>>>
}

impl Server {
    
    pub fn start() -> Result<Self> {
        let mqtt_handler = tokio::spawn(async { mqtt::start_mqtt_server().await });
        let http_handler = tokio::spawn(async { http::start_http_server().await });
        let handler = try_join_all(vec![mqtt_handler, http_handler]);
        Ok(Server {handler})
        
    }
    
    pub async fn start_web_server() -> Result<()> {
        
        Ok(())
    }
}


