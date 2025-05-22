pub mod server;
mod mqtt;
mod http;
mod discovery;

use std::ffi::{c_char, CStr, CString};
use std::fmt::Display;
use std::sync::{OnceLock};
use crate::server::{Server, ServerConfig};
use once_cell::sync::Lazy;
use serde_json::json;
use tokio::runtime::Runtime;
use crate::discovery::discovery;

pub struct MyHandle(pub(crate) Server);

static SERVER: OnceLock<MyHandle> = OnceLock::new();



static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| {
    Runtime::new().expect("Failed to create Tokio runtime")
});

#[repr(C)]
#[derive(Debug)]
pub enum ErrorCode {
    Ok = 0,
    BadConfig = 1,
    StartServerError = 2,
    InvalidServerPoint = 3,
    ServerHasInit = 4,
    MDNSInitFailure = 5,
    OutOfAllocatedBounds = 6,
}
impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn tiny_protocol_get_config(output:*mut c_char) -> ErrorCode {
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

/*
Attention: it would block current thread for *seconds*
*/
#[unsafe(no_mangle)]
pub extern "C" fn tiny_protocol_discovery(service: *const c_char, seconds: u64, output_str: *mut c_char, output_str_len: usize) -> ErrorCode {
    let service = unsafe {
        CStr::from_ptr(service).to_string_lossy().into_owned()
    };
    match discovery(&service, seconds)  {
        Ok(services) => {
            //  todo: serialize services
            let j = json!(services.iter().map(|service|{
                let addresses = json!(service.get_addresses().iter().map(|addr|{
                    addr.to_string()
                }).collect::<Vec<_>>());
                let properties = service.get_properties().iter().map(|property| {
                   json!({
                        "key": property.key(),
                        "value": property.val_str(),
                    })
                }).collect::<Vec<_>>(); 
                
                json!({
                    "hostname": service.get_hostname(), 
                    "addresses": addresses,
                    "port": service.get_port(),
                    "properties": properties,
                })
            }).collect::<Vec<_>>());
            let j_str = CString::new(j.to_string()).unwrap();
            let j_str = j_str.as_bytes_with_nul();
            
            if j_str.len() > output_str_len {
                return ErrorCode::OutOfAllocatedBounds;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(j_str.as_ptr() as *const c_char, output_str, j_str.len());
            }
            ErrorCode::Ok
        }
        Err(e) => {
            e
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tiny_protocol_start_server(config: *const c_char)  -> ErrorCode {
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