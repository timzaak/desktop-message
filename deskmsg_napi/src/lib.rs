#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use napi::{Result, Status};
use deskmsg::server::{Server, ServerConfig as DeskmsgServerConfig};
use once_cell::sync::Lazy;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

// Remove the example sum function

#[napi(object)]
pub struct NapiServerConfig {
  pub mqtt_address: String,
  pub http_address: String,
  pub basic_path: String,
  pub http_auth_token: String,
}

#[napi(object)]
pub struct NapiProperty {
  pub key: String,
  pub value: String,
}

#[napi(object)]
pub struct NapiServiceInfo {
  pub fullname: String,
  pub hostname: String,
  pub port: u16,
  pub addresses: Vec<String>,
  pub properties: Vec<NapiProperty>,
}

static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| {
  Runtime::new().expect("Failed to create Tokio runtime for NAPI module")
});

static SERVER: OnceLock<Server> = OnceLock::new();

#[napi]
pub fn start_server(config: NapiServerConfig) -> Result<()> {
  if SERVER.get().is_some() {
    return Err(napi::Error::new(Status::GenericFailure, "Server already initialized".to_string()));
  }

  let deskmsg_config = DeskmsgServerConfig {
    mqtt_address: config.mqtt_address,
    http_address: config.http_address,
    basic_path: config.basic_path,
    http_auth_token: config.http_auth_token,
  };

  let _guard = TOKIO_RT.enter();
  match Server::new(deskmsg_config) {
    Ok(server_instance) => {
      if SERVER.set(server_instance).is_err() {
        log::error!("NAPI: Failed to set server instance after check");
        return Err(napi::Error::new(Status::GenericFailure, "Failed to store server instance".to_string()));
      }
      Ok(())
    }
    Err(e) => {
      log::error!("NAPI: Error starting server: {}", e);
      Err(napi::Error::new(Status::GenericFailure, format!("Error starting server: {}", e)))
    }
  }
}

#[napi]
pub fn get_config() -> Result<NapiServerConfig> {
  match SERVER.get() {
    Some(server_instance) => {
      let config = server_instance.get_config(); // Assumes get_config() returns DeskmsgServerConfig
      Ok(NapiServerConfig {
        mqtt_address: config.mqtt_address,
        http_address: config.http_address,
        basic_path: config.basic_path,
        http_auth_token: config.http_auth_token,
      })
    }
    None => Err(napi::Error::new(Status::GenericFailure, "Server not initialized".to_string())),
  }
}

#[napi]
pub fn discovery_mdns(service_name: String, seconds: u32) -> Result<Vec<NapiServiceInfo>> {
  let result = TOKIO_RT.block_on(async {
    deskmsg::discovery::discovery_mdns(&service_name, seconds as u64)
  });

  match result {
    Ok(services) => {
      let mut napi_services = Vec::new();
      for service_info in services {
        let addresses = service_info.get_addresses().iter().map(ToString::to_string).collect();
        let properties = service_info.get_properties().iter().map(|p| NapiProperty {
          key: p.key().to_string(),
          value: p.val_str().to_string(),
        }).collect();

        napi_services.push(NapiServiceInfo {
          fullname: service_info.get_fullname().to_string(),
          hostname: service_info.get_hostname().to_string(),
          port: service_info.get_port(),
          addresses,
          properties,
        });
      }
      Ok(napi_services)
    }
    Err(e) => {
      log::error!("NAPI: MDNS Discovery failed: {}", e);
      Err(napi::Error::new(Status::GenericFailure, format!("MDNS Discovery failed: {}", e)))
    }
  }
}