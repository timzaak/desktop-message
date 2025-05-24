use std::ffi::c_char;
use deskmg::ErrorCode; // Uses ErrorCode from the deskmg crate

#[no_mangle]
pub extern "C" fn tiny_protocol_get_config(output: *mut c_char) -> ErrorCode {
    deskmg::api_get_config(output)
}

#[no_mangle]
pub extern "C" fn tiny_protocol_discovery(service: *const c_char, seconds: u64, output_str: *mut c_char, output_str_len: usize) -> ErrorCode {
    deskmg::api_discovery(service, seconds, output_str, output_str_len)
}

#[no_mangle]
pub extern "C" fn tiny_protocol_start_server(config: *const c_char) -> ErrorCode {
    deskmg::api_start_server(config)
}
