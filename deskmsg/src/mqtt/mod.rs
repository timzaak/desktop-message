use ahash::HashMapExt;
use anyhow::Result;
use rmqtt::context::ServerContext;
use rmqtt::net::{Builder, Listener};
use rmqtt::server::MqttServer as RMqttServer;
use rmqtt::types::HashMap;
use std::net::SocketAddr;

mod acl;

pub struct MqttServer {}

impl MqttServer {
    pub fn try_bind(address: SocketAddr) -> Result<(SocketAddr, Listener)> {
        let socket = Builder::new().name("external/tcp").laddr(address.clone()).bind()?;
        let socket = socket.tcp()?;
        let address = socket.local_addr()?;
        Ok((address, socket))
    }

    pub async fn start_rmqtt_server(listener: Listener) -> Result<()> {
        let mut plugin_config = HashMap::new();
        plugin_config.insert("auto-subscription".to_owned(), r#"subscribes = []"#.to_owned());

        let scx = ServerContext::new()/*.plugins_config_map(plugin_config)*/.build().await;

        acl::register_named(&scx, "acl", true, false).await?;
        // subscribe server client connection $SYS/brokers/+/clients/+/#
        rmqtt_sys_topic::register_named(&scx, "sys-topic", true, false).await?;
        //rmqtt_auto_subscription::register_named(&scx, "auto-subscription", true, false).await?;

        RMqttServer::new(scx).listener(listener).build().run().await?;
        Ok(())
    }
}
