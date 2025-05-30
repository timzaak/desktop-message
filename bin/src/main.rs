use std::ffi::CString;
use std::time::Duration;
use tracing::Level;
use tracing_subscriber::EnvFilter;
use deskmsg::server::ServerConfig;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(Level::INFO.into())
                .from_env_lossy(),
        )
        .init();


    let config = ServerConfig {
        mqtt_address: "0.0.0.0:0".to_string(),
        http_address: "0.0.0.0:0".to_string(),
        basic_path: "".to_string(),
        http_auth_token: "default_token_from_main".to_string(), // Added new field
    };
    let config_str = serde_json::to_string(&config)?;
    
    let config_str  = CString::new(config_str)?;
    
    deskmsg_c::deskmsg_start_server(config_str.as_ptr());

    // let rt  = tokio::runtime::Runtime::new()?;
    // rt.block_on(async {
    //     let server = deskmsg::server::Server::new(config).unwrap();
    // });
    //server.get_config()

    std::thread::sleep(Duration::from_secs(60*300));
    
    Ok(())
}


