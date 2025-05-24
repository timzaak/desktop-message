use napi_derive::napi;
use napi::Result;
use napi::{Error, Status};

// Import the necessary items from the deskmsg_c crate.
// We will call the Rust functions directly, not the C extern functions.
// This requires that the functions and types we need are public in deskmsg_c.
// Assuming ServerConfig, ErrorCode, discovery, Server are accessible from deskmsg_c crate root 
// or its public modules.
// We also need access to TOKIO_RT and SERVER from deskmsg_c.

use deskmsg_c::{ServerConfig, ErrorCode, MyHandle}; // MyHandle might not be needed if we don't interact with it directly
use deskmsg_c::TOKIO_RT; // Assuming this is pub in deskmsg_c
use deskmsg_c::SERVER; // Assuming this is pub in deskmsg_c

// Helper to convert deskmsg_c::ErrorCode to napi::Error
fn to_napi_error(ec: deskmsg_c::ErrorCode) -> Error {
    Error::new(Status::GenericFailure, format!("deskmsg_c error: {:?}", ec))
}

#[napi(js_name = "startServer")]
pub fn start_server(config_json: String) -> Result<()> {
    // Check if server is already running using the SERVER static from deskmsg_c
    if SERVER.get().is_some() {
        return Err(Error::new(Status::GenericFailure, "Server is already initialized."));
    }

    let config: ServerConfig = serde_json::from_str(&config_json)
        .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid JSON config: {}", e)))?;

    // The original C function `tiny_protocol_start_server` uses `TOKIO_RT.enter()`
    // and then calls Server::new. We should replicate this by running Server::new 
    // within the context of deskmsg_c's Tokio runtime.
    let result = TOKIO_RT.block_on(async {
        // This is a simplified conceptual call. The actual `Server::new` and `SERVER.set` logic 
        // is inside `tiny_protocol_start_server`. 
        // We should ideally call a Rust function in `deskmsg_c` that encapsulates this logic
        // if `tiny_protocol_start_server` itself is not directly callable with Rust types or 
        // if its internal logic is too tied to C types.
        
        // For now, let's assume there's a Rust equivalent or we adapt.
        // The original C function `tiny_protocol_start_server` takes a c_char for config.
        // We need to call the underlying Rust logic. 
        // Let's mimic the logic of tiny_protocol_start_server's Rust implementation part.

        match deskmsg_c::Server::new(config) { // Assuming deskmsg_c::Server::new is public and takes ServerConfig
            Ok(server_instance) => {
                let handler = MyHandle(server_instance); // MyHandle must be public
                if SERVER.set(handler).is_err() {
                    Err(deskmsg_c::ErrorCode::StartServerError) // Or some internal error
                } else {
                    Ok(deskmsg_c::ErrorCode::Ok)
                }
            }
            Err(e) => {
                // Log the error if possible, e.g., using log crate if deskmsg_c uses it
                // log::error!("Failed to start server: {}", e);
                // The error `e` here is likely already an ErrorCode or similar from deskmsg_c's Server::new
                // If Server::new returns a Result<Server, ErrorCode>, then this is fine.
                // If it returns another error type, it needs conversion.
                // For now, assuming it's an ErrorCode or can be mapped to it.
                Err(deskmsg_c::ErrorCode::StartServerError) // Placeholder, map error properly
            }
        }
    });

    match result {
        Ok(ErrorCode::Ok) => Ok(()),
        Ok(ec) | Err(ec) => Err(to_napi_error(ec)), // Handle both Ok(Error) and Err(Error)
    }
}

#[napi(js_name = "getConfig")]
pub fn get_config() -> Result<String> {
    if let Some(server_handle) = SERVER.get() {
        // Accessing fields of server_handle.0 (Server instance)
        let config_data = ServerConfig {
            mqtt_address: server_handle.0.mqtt_address.to_string(),
            http_address: server_handle.0.http_address.to_string(),
            basic_path: "".to_owned(), // Assuming basic_path is not part of running config or set to default
        };
        serde_json::to_string(&config_data)
            .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to serialize config: {}", e)))
    } else {
        Err(Error::new(Status::GenericFailure, "Server not initialized or config not available."))
    }
}


#[napi]
pub fn discovery(service_name: String, seconds: u64) -> Result<String> {
    // The original `tiny_protocol_discovery` calls `deskmsg::discovery::discovery`.
    // We can call that directly if it's accessible and takes appropriate Rust types.
    // `deskmsg::discovery::discovery` is what we should aim to call.

    // Assuming `deskmsg::discovery::discovery` is available at `deskmsg_c::discovery::discovery` 
    // or re-exported, or we add `deskmsg` crate as a dependency to `deskmsg_napi` too.
    // For now, let's assume it's available via `deskmsg_c` re-export or direct call if `deskmsg_c` exposes it.
    // The C function `tiny_protocol_discovery` has complex logic for JSON serialization.
    // It would be best if `deskmsg_c` provided a Rust function that returns the `Vec<ServiceInfo>` or the final JSON string.

    // Let's call the C-level function from deskmsg_c for now, but manage memory for output. 
    // This is less ideal than calling a pure Rust function from deskmsg_c.
    // A better approach: deskmsg_c should have a Rust function that returns Result<String, ErrorCode>.

    // Simplification: Assume deskmsg_c::discovery_to_json_string(service_name: &str, seconds: u64) -> Result<String, ErrorCode>
    // If such function doesn't exist, this will need more work in deskmsg_c or here.
    // For now, we will replicate the logic of `tiny_protocol_discovery`'s Rust part:

    match deskmsg_c::discovery::discovery(&service_name, seconds) { // Assuming this path is valid
        Ok(services) => {
            let j = serde_json::json!(services.iter().map(|service_info|{ 
                let addresses = serde_json::json!(service_info.get_addresses().iter().map(|addr|{
                    addr.to_string()
                }).collect::<Vec<_>>());
                let properties = service_info.get_properties().iter().map(|property| {
                   serde_json::json!({
                        "key": property.key(),
                        "value": property.val_str(),
                    })
                }).collect::<Vec<_>>();
                
                serde_json::json!({
                    "hostname": service_info.get_hostname(), 
                    "addresses": addresses,
                    "port": service_info.get_port(),
                    "properties": properties,
                })
            }).collect::<Vec<_>>());
            
            Ok(j.to_string())
        }
        Err(e) => Err(to_napi_error(e)), // Assuming the error 'e' is deskmsg_c::ErrorCode
    }
}

// Optional: Add a sync_test function to verify basic operation if needed
#[napi]
pub fn sync_test(val: u32) -> u32 {
    val + 100
}
