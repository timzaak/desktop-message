use deskmsg::server::ServerConfig;
use std::time::Duration;
use tracing::Level;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder().with_default_directive(Level::INFO.into()).from_env_lossy())
        .init();

    let config = ServerConfig {
        mqtt_address: "0.0.0.0:1883".to_string(),
        http_address: "0.0.0.0:8088".to_string(),
        basic_path: "".to_string(),
        http_auth_token: "default_token_from_main".to_string(), // Added new field
    };

    let rt  = tokio::runtime::Runtime::new()?;
    let server = rt.block_on(async {
        let server = deskmsg::server::Server::new(config).unwrap();
        server
    });
    println!("{:?}",server.get_config());
    
    std::thread::sleep(Duration::from_secs(60 * 300));

    Ok(())
}
