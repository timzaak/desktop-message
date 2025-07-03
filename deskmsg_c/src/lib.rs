mod c_peripheral;

use crate::c_peripheral::CPeripheral;
use btleplug::api::Peripheral;
use deskmsg::discovery::{ble_write, discover_ble_devices, discovery_mdns};
use deskmsg::server::{Server, ServerConfig};
use log;
use once_cell::sync::Lazy;
use serde_json::json;
use std::ffi::{CStr, CString, c_char, c_uint};
use std::fmt::Display;
use std::sync::OnceLock;
use tokio::runtime::Runtime;

pub struct MyHandle(pub Server);

pub static SERVER: OnceLock<MyHandle> = OnceLock::new();

pub static TOKIO_RT: Lazy<Runtime> = Lazy::new(|| {
    // Made public
    Runtime::new().expect("Failed to create Tokio runtime")
});

#[repr(C)]
#[derive(Debug)]
pub enum ErrorCode {
    // Already public, no change needed here based on instruction
    Ok = 0,
    BadConfig = 1,
    StartServerError = 2,
    InvalidServerPoint = 3,
    ServerHasInit = 4,
    MDNSInitFailure = 5,
    OutOfAllocatedBounds = 6,
    OperationFailure = 7,
}
impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Changed to {:?} to use Debug derive, as per typical Display for enums. Or keep as self.to_string() if specific string representations are defined. For now, using Debug.
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_get_config(output_ptr: *mut c_char) -> ErrorCode {
    if let Some(server_handle) = SERVER.get() {
        let str_config = json!({
            "mqtt_address": server_handle.0.mqtt_address.to_string(),
            "http_address": server_handle.0.http_address.to_string(),
        })
        .to_string();

        let c_str = CString::new(str_config).unwrap(); // Consider error handling for CString::new
        let bytes = c_str.as_bytes_with_nul();
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, output_ptr, bytes.len());
        }
        ErrorCode::Ok
    } else {
        ErrorCode::InvalidServerPoint
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_discovery_mdns(
    service_ptr: *const c_char,
    seconds: u64,
    output_str_ptr: *mut c_char,
    output_str_len: usize,
) -> ErrorCode {
    let service_name = unsafe {
        // Renamed service to service_name for clarity
        CStr::from_ptr(service_ptr).to_string_lossy().into_owned()
    };

    match discovery_mdns(&service_name, seconds) {
        Ok(services) => {
            let j = json!(
                services
                    .iter()
                    .map(|service_info| {
                        // Renamed service to service_info
                        let addresses = json!(
                            service_info.get_addresses().iter().map(|addr| { addr.to_string() }).collect::<Vec<_>>()
                        );
                        let properties = service_info
                            .get_properties()
                            .iter()
                            .map(|property| {
                                json!({
                                    "key": property.key(),
                                    "value": property.val_str(),
                                })
                            })
                            .collect::<Vec<_>>();

                        json!({
                            "hostname": service_info.get_hostname(),
                            "addresses": addresses,
                            "port": service_info.get_port(),
                            "properties": properties,
                        })
                    })
                    .collect::<Vec<_>>()
            );

            let j_str = CString::new(j.to_string()).unwrap();
            let j_bytes = j_str.as_bytes_with_nul();

            if j_bytes.len() > output_str_len {
                return ErrorCode::OutOfAllocatedBounds;
            }
            unsafe {
                std::ptr::copy_nonoverlapping(j_bytes.as_ptr() as *const c_char, output_str_ptr, j_bytes.len());
            }
            ErrorCode::Ok
        }
        Err(_) => ErrorCode::MDNSInitFailure,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_discovery_ble_scan(
    service_uuid_ptr: *const c_char,
    seconds: u32,
    peripherals: *mut *mut CPeripheral,
    len: *mut c_uint,
) -> ErrorCode {
    let service_uuid = if service_uuid_ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(service_uuid_ptr).to_string_lossy().into_owned() })
    };
    let result = TOKIO_RT.block_on(async { discover_ble_devices(service_uuid.as_deref(), seconds as u64).await });
    match result {
        Ok(devices) => {
            let mut c_devices =
                devices.into_iter().map(|device| CPeripheral { peripheral: device }).collect::<Vec<_>>();
            c_devices.shrink_to_fit();
            let _len = c_devices.len() as c_uint;
            let ptr = c_devices.as_mut_ptr();
            std::mem::forget(c_devices);
            unsafe {
                *len = _len;
                *peripherals = ptr;
            }
            ErrorCode::Ok
        }
        Err(e) => {
            log::warn!("Error: {}", e);
            ErrorCode::OperationFailure
        }
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_discovery_ble_free(structs: *mut CPeripheral, len: c_uint) {
    if structs.is_null() {
        return;
    }
    let _ = unsafe { Vec::from_raw_parts(structs, len as usize, len as usize) };
}

#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_discovery_ble_write(
    cperipheral: *mut CPeripheral,
    service_uuid: *const c_char,
    characteristic_uuid: *const c_char,
    message: *const c_char,
) -> ErrorCode {
    let cperipheral = unsafe { &*cperipheral };
    let service_uuid = unsafe { CStr::from_ptr(service_uuid).to_string_lossy().into_owned() };
    let characteristic_uuid = unsafe { CStr::from_ptr(characteristic_uuid).to_string_lossy().into_owned() };
    let message = unsafe { CStr::from_ptr(message).to_string_lossy().into_owned() };
    let result = TOKIO_RT
        .block_on(async { ble_write(&cperipheral.peripheral, service_uuid, characteristic_uuid, message).await });
    match result {
        Ok(_) => ErrorCode::Ok,
        Err(e) => {
            log::warn!("Error: {}", e);
            ErrorCode::OperationFailure
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_start_server(config_ptr: *const c_char) -> ErrorCode {
    if SERVER.get().is_some() {
        return ErrorCode::ServerHasInit;
    }

    let config_str = unsafe {
        // Renamed config to config_str
        CStr::from_ptr(config_ptr).to_string_lossy().into_owned()
    };

    match serde_json::from_str::<ServerConfig>(&config_str) {
        Ok(parsed_config) => {
            // Renamed config to parsed_config
            let _guard = TOKIO_RT.enter(); // Renamed _guid to _guard for clarity

            match Server::new(parsed_config) {
                // Ensure Server::new exists and takes ServerConfig
                Ok(server_instance) => {
                    // Renamed server to server_instance
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
