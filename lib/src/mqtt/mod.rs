use std::net::SocketAddr;
use rmqtt::context::ServerContext;
use rmqtt::net::{Builder, Listener};
use rmqtt::server::{MqttServer as RMqttServer};
use anyhow::Result;

mod acl;


pub struct MqttServer {}

impl MqttServer {
    //TODO: address could not modify
    // https://github.com/rmqtt/rmqtt/issues/194
    pub fn try_bind(address:SocketAddr) -> Result<(SocketAddr, Listener)>{
        let socket = Builder::new().name("external/tcp").laddr(address.clone()).bind()?;
        let socket = socket.tcp()?;
        Ok((address, socket))
    }

    pub async fn start_rmqtt_server(listener:Listener) -> Result<()>{
        let scx = ServerContext::new().build().await;
        acl::register_named(&scx, "acl", true, false).await?;
        rmqtt_sys_topic::register_named(&scx, "sys-topic", true, false).await?;
        
        RMqttServer::new(scx)
            .listener(listener)
            .build()
            .run()
            .await?;
        Ok(())
    }
}
