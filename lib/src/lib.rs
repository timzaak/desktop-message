pub mod server;
mod mqtt;
mod http;

use std::ffi::{c_char, CStr, CString};
use std::sync::{OnceLock};
use crate::server::{Server, ServerConfig};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub struct MyHandle(pub(crate) Server);

static SERVER: OnceLock<MyHandle> = OnceLock::new();



static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().expect("Failed to create Tokio runtime")
});

#[repr(C)]
pub enum ErrorCode {
    Ok = 0,
    BadConfig = 1,
    StartServerError = 2,
    InvalidServerPoint = 3,
    ServerHasInit = 4,
}


#[unsafe(no_mangle)]
pub extern "C" fn tiny_rmqtt_get_config(output:*mut c_char) -> ErrorCode {
    if let Some(server) = SERVER.get() {
        let config = ServerConfig {
            mqtt_address: server.0.mqtt_address.to_string(),
            http_address: server.0.http_address.to_string(),
            basic_path: "".to_owned()
        };
        return match serde_json::to_string(&config) {
            Ok(str_config) => unsafe {
                let c_str = CString::new(str_config).unwrap();
                let bytes = c_str.as_bytes_with_nul();
                std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, output, bytes.len());
                ErrorCode::Ok
            }
            Err(_) => ErrorCode::BadConfig
        }
    }
    ErrorCode::InvalidServerPoint
}

#[unsafe(no_mangle)]
pub extern "C" fn tiny_rmqtt_start_server(config: *const c_char)  -> ErrorCode {
    if SERVER.get().is_some() {
        return ErrorCode::ServerHasInit;
    }
    // convert config config_len to Rust String
    let config = unsafe {
        let config = CStr::from_ptr(config).to_string_lossy().into_owned();
        config
    };

    match serde_json::from_str::<ServerConfig>(&config) {
         Ok(config) => {
             let _guid = TOKIO_RT.enter();
             match Server::new(config){
                 Ok(server) => {
                     let handler = MyHandle(server);
                     let _ = SERVER.set(handler);
                     ErrorCode::Ok
                 }

                 Err(e) => {
                     log::error!("Error starting server: {}", e);
                     ErrorCode::StartServerError
                 }
             }
        }
        Err(e) => {
            log::error!("Error parsing config: {}", e);
            ErrorCode::BadConfig
        }
    }

}