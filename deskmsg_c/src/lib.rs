use std::ffi::{c_char, CStr, CString};
use std::sync::OnceLock;
use deskmsg::ErrorCode;
use deskmsg::server::{Server, ServerConfig};
use serde_json::json;
use tokio::runtime::Runtime;
use deskmsg::discovery::discovery;
use log;
use once_cell::sync::Lazy;

pub struct MyHandle(pub Server);

pub static SERVER: OnceLock<MyHandle> = OnceLock::new();

pub static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| { // Made public
    Runtime::new().expect("Failed to create Tokio runtime")
});

#[unsafe(no_mangle)]
pub extern "C" fn tiny_protocol_get_config(output_ptr: *mut c_char) -> ErrorCode {
    if let Some(server_handle) = SERVER.get() {
        let config = ServerConfig {
            mqtt_address: server_handle.0.mqtt_address.to_string(),
            http_address: server_handle.0.http_address.to_string(),
            basic_path: "".to_owned()
        };
        match serde_json::to_string(&config) {
            Ok(str_config) => {
                let c_str = CString::new(str_config).unwrap(); // Consider error handling for CString::new
                let bytes = c_str.as_bytes_with_nul();
                unsafe {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, output_ptr, bytes.len());
                }
                ErrorCode::Ok
            }
            Err(_) => ErrorCode::BadConfig
        }
    } else {
        ErrorCode::InvalidServerPoint
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tiny_protocol_discovery(service_ptr: *const c_char, seconds: u64, output_str_ptr: *mut c_char, output_str_len: usize) -> ErrorCode {
    let service_name = unsafe { // Renamed service to service_name for clarity
        CStr::from_ptr(service_ptr).to_string_lossy().into_owned()
    };

    match discovery(&service_name, seconds) {
        Ok(services) => {
            let j = json!(services.iter().map(|service_info|{ // Renamed service to service_info
                let addresses = json!(service_info.get_addresses().iter().map(|addr|{
                    addr.to_string()
                }).collect::<Vec<_>>());
                let properties = service_info.get_properties().iter().map(|property| {
                   json!({
                        "key": property.key(),
                        "value": property.val_str(),
                    })
                }).collect::<Vec<_>>();
                
                json!({
                    "hostname": service_info.get_hostname(), 
                    "addresses": addresses,
                    "port": service_info.get_port(),
                    "properties": properties,
                })
            }).collect::<Vec<_>>());
            
            let j_str = CString::new(j.to_string()).unwrap(); // Consider error handling
            let j_bytes = j_str.as_bytes_with_nul(); // Renamed j_str to j_bytes for clarity

            if j_bytes.len() > output_str_len {
                return ErrorCode::OutOfAllocatedBounds;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(j_bytes.as_ptr() as *const c_char, output_str_ptr, j_bytes.len());
            }
            ErrorCode::Ok
        }
        Err(e) => {
            e // Assuming discovery function returns ErrorCode on error
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tiny_protocol_start_server(config_ptr: *const c_char) -> ErrorCode {
    
    if SERVER.get().is_some() {
        return ErrorCode::ServerHasInit;
    }

    let config_str = unsafe { // Renamed config to config_str
        CStr::from_ptr(config_ptr).to_string_lossy().into_owned()
    };


    match serde_json::from_str::<ServerConfig>(&config_str) {
        Ok(parsed_config) => { // Renamed config to parsed_config
            let _guard = TOKIO_RT.enter(); // Renamed _guid to _guard for clarity

            match Server::new(parsed_config){ // Ensure Server::new exists and takes ServerConfig
                Ok(server_instance) => { // Renamed server to server_instance
                    let handler = MyHandle(server_instance);
                    if SERVER.set(handler).is_err() {
                        // This case should ideally not happen if the initial SERVER.get().is_some() check is correct
                        // log::error!("Failed to set server instance after check"); // Optional: log this unexpected state
                        return ErrorCode::StartServerError; // Or a more specific error
                    }
                    ErrorCode::Ok
                }
                Err(e) => {
                    // Ensure `use log;`
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
