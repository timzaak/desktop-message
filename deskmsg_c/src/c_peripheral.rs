use btleplug::api::Peripheral;
use std::ffi::{c_char, c_int};

pub struct CPeripheral {
    pub peripheral: btleplug::platform::Peripheral,
    //    self.peripheral.properties();
    //    self.peripheral.services();
}

fn to_c_char(rust_str: &str, buf: *mut c_char, len: c_int) -> c_int {
    let required_len = rust_str.len() + 1;
    if len < required_len as c_int {
        return -1; // 缓冲区太小
    }
    unsafe {
        std::ptr::copy_nonoverlapping(rust_str.as_ptr() as *const c_char, buf, rust_str.len());
        *buf.add(rust_str.len()) = 0;
    }

    required_len as c_int
}

#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_ble_id(
    peripheral: *mut btleplug::platform::Peripheral,
    buf: *mut c_char,
    len: c_int,
) -> c_int {
    let peripheral = unsafe { &*peripheral };
    let rust_str = peripheral.id().to_string();
    to_c_char(&rust_str, buf, len)
}

#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_ble_address(
    peripheral: *mut btleplug::platform::Peripheral,
    buf: *mut c_char,
    len: c_int,
) -> c_int {
    let peripheral = unsafe { &*peripheral };
    let rust_str = peripheral.address().to_string();
    to_c_char(&rust_str, buf, len)
}
/*

// need tokio runtime
#[unsafe(no_mangle)]
pub extern "C" fn deskmsg_ble_properties(peripheral: *mut btleplug::platform::Peripheral, buf: *mut c_char, len: c_int) -> c_int {
    let peripheral = unsafe { &*peripheral };
    //let properties = peripheral.properties();
    todo!()

}


 */
