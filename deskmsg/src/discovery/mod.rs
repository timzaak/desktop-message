pub mod ble;
pub mod mdns;

pub use mdns::discovery_mdns;

pub use ble::ble_write;
pub use ble::discover_ble_devices;
