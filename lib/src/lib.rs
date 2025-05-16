pub mod server;
mod mqtt;
mod http;

use std::ffi::{c_char, CString};



#[repr(transparent)]
pub struct MyHandle(pub(crate) Box<server::Server>);
pub type MyHandlePtr = *mut MyHandle;

/// 安全检查宏
macro_rules! check_handle {
    ($handle:expr) => {
        if $handle.is_null() {
            return Default::default(); // 或适当的错误处理
        }
        &mut *$handle
    };
}


#[unsafe(no_mangle)]
pub extern "C" fn free_rust_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            // 将指针重新转换为 CString 然后 drop
            let _ = CString::from_raw(ptr);
        }
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn rust_function() {
    println!("Hello World!");
}