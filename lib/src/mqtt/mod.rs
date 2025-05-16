use rmqtt::context::ServerContext;
use rmqtt::net::Builder;
use rmqtt::server::MqttServer;
use anyhow::Result;

mod acl;



pub async fn start_mqtt_server() -> Result<()>{
    let scx = ServerContext::new().build().await;
    acl::register_named(&scx, "acl", true, false).await?;
    MqttServer::new(scx)
        .listener(Builder::new().name("external/tcp").laddr(([0, 0, 0, 0], 1883).into()).bind()?.tcp()?)
        .build()
        .run()
        .await?;
    Ok(())
}