use anyhow::Result;
use btleplug::api::{Central, Manager as _, ScanFilter, WriteType};
use btleplug::platform::Manager;
use log;
use std::time::Duration;
use uuid::Uuid;

/// Discovers BLE devices for a short duration, optionally filtering by service UUID.
pub async fn discover_ble_devices(
    service_uuid_str: Option<&str>,
    seconds: u64,
) -> Result<Vec<btleplug::platform::Peripheral>> {
    let manager = Manager::new().await.map_err(|e| anyhow::anyhow!("Failed to create BLE manager: {}", e))?;

    let adapters = manager.adapters().await.map_err(|e| anyhow::anyhow!("Failed to list BLE adapters: {}", e))?;

    let central = match adapters.into_iter().next() {
        Some(adapter) => adapter,
        None => {
            log::warn!("No BLE adapters found.");
            return Ok(Vec::new());
        }
    };
    log::info!("Starting BLE scan on adapter: {:?}...", central.adapter_info().await.unwrap_or_default());

    // Create ScanFilter
    let scan_filter = if let Some(uuid_str) = service_uuid_str {
        match Uuid::parse_str(uuid_str) {
            Ok(parsed_uuid) => {
                log::debug!("Scanning with service UUID filter: {}", parsed_uuid);
                ScanFilter { services: vec![parsed_uuid] }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Invalid service UUID string '{}': {}", uuid_str, e));
            }
        }
    } else {
        log::debug!("Scanning with default filter (all devices).");
        ScanFilter::default()
    };

    // Start scanning for devices
    central.start_scan(scan_filter).await.map_err(|e| anyhow::anyhow!("Failed to start scan: {}", e))?;

    tokio::time::sleep(Duration::from_secs(seconds)).await;

    let result = central.peripherals().await.map_err(|e| anyhow::anyhow!("Failed to get peripherals: {}", e))?;
    central.stop_scan().await.map_err(|e| anyhow::anyhow!("Failed to stop scan: {}", e))?;
    Ok(result)
}

use btleplug::api::Peripheral;
pub async fn ble_write(
    peripheral: &btleplug::platform::Peripheral,
    service_name: String,
    characteristic_name: String,
    message: String,
) -> Result<()> {
    if !peripheral.is_connected().await? {
        peripheral.connect().await.map_err(|e| anyhow::anyhow!("Failed to connect to peripheral: {}", e))?;
        peripheral.discover_services().await?
    }

    let v = peripheral.services();
    for service in v {
        if service.uuid.to_string().contains(&service_name) {
            log::info!("Found service: {}", service.uuid);
            let characteristics = service.characteristics;
            for characteristic in characteristics {
                if characteristic.uuid.to_string().contains(&characteristic_name) {
                    log::info!("Found characteristic: {}", characteristic.uuid);
                    peripheral.write(&characteristic, message.as_bytes(), WriteType::WithResponse).await?;
                    peripheral.disconnect().await?;
                    return Ok(());
                }
            }
        }
    }
    peripheral.disconnect().await?;
    Err(anyhow::anyhow!("No service or characteristic found."))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: This test will require a Bluetooth adapter to be active on the system.
    // It might also require specific permissions depending on the OS.
    // In a CI environment without Bluetooth hardware, this test will likely fail or be skipped.
    #[tokio::test]
    async fn test_discover_ble_devices_execution() {
        // Simple test to ensure the function executes without panicking.
        // It doesn't assert specific devices as that's hardware dependent.
        match discover_ble_devices(None, 5).await {
            Ok(devices) => {
                println!("Test (no filter) discovered {} devices:", devices.len());
                for device_info in devices {
                    println!("  - {}", device_info);
                }
            }
            Err(e) => {
                log::warn!(
                    "test_discover_ble_devices_execution (no filter) error: {}. This might be due to no available Bluetooth adapter.",
                    e
                );
            }
        }
    }

    #[tokio::test]
    async fn test_discover_ble_devices_with_filter() {
        let dummy_service_uuid = "00001800-0000-1000-8000-00805f9b34fb"; // Example: Generic Access Service
        match discover_ble_devices(Some(dummy_service_uuid), 5).await {
            Ok(devices) => {
                log::info!(
                    "Test with filter discovered {} devices. (Expected to be few or none unless service is present).",
                    devices.len()
                );
                for device_info in devices {
                    log::info!("  - {}", device_info);
                }
            }
            Err(e) => {
                log::warn!(
                    "Test with filter error: {}. This might be due to no adapter or invalid UUID format if not careful.",
                    e
                );
            }
        }
    }
}
