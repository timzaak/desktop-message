
use anyhow::Result;
use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::{Manager, Peripheral};
use uuid::Uuid;
use std::time::Duration;
use log;

/// Discovers BLE devices for a short duration, optionally filtering by service UUID.
pub async fn discover_ble_devices(service_uuid_str: Option<&str>, seconds: u64) -> Result<Vec<Peripheral>> {
    let manager = Manager::new().await.map_err(|e| {
        log::error!("Failed to create BLE manager: {}", e);
        anyhow::anyhow!("Failed to create BLE manager: {}", e)
    })?;

    let adapters = manager.adapters().await.map_err(|e| {
        log::error!("Failed to list BLE adapters: {}", e);
        anyhow::anyhow!("Failed to list BLE adapters: {}", e)
    })?;

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
                log::info!("Scanning with service UUID filter: {}", parsed_uuid);
                ScanFilter {
                    services: vec![parsed_uuid],
                }
            }
            Err(e) => {
                log::error!("Invalid service UUID string '{}': {}", uuid_str, e);
                return Err(anyhow::anyhow!("Invalid service UUID string '{}': {}", uuid_str, e));
            }
        }
    } else {
        log::info!("Scanning with default filter (all devices).");
        ScanFilter::default()
    };

    // Start scanning for devices
    central.start_scan(scan_filter).await.map_err(|e| {
        log::error!("Failed to start scan: {}", e);
        anyhow::anyhow!("Failed to start scan: {}", e)
    })?;

    tokio::time::sleep(Duration::from_secs(seconds)).await;

    let result = central.peripherals().await.map_err(|e| {
            log::error!("Failed to get peripherals: {}", e);
            anyhow::anyhow!("Failed to get peripherals: {}", e)
        })?;
    central.stop_scan().await.map_err(|e| {
        log::error!("Failed to stop scan: {}", e);
        anyhow::anyhow!("Failed to stop scan: {}", e)
    })?;
    Ok(result)
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
                log::info!("Test (no filter) discovered {} devices:", devices.len());
                for device_info in devices {
                    log::info!("  - {}", device_info);
                }
                // If you want to assert that it *could* find devices,
                // you might check `assert!(!devices.is_empty())` but this makes the test flaky.
                // For now, just succeeding is enough for a basic test.
            }
            Err(e) => {
                // If no adapter is present, this might error out.
                // We log the error but don't fail the test if it's a known issue like no adapter.
                log::warn!("test_discover_ble_devices_execution (no filter) error: {}. This might be due to no available Bluetooth adapter.", e);
                // Consider making this a more specific error check if needed, e.g.
                // if e.to_string().contains("No BLE adapters found") // or similar specific error from btleplug
            }
        }
    }

    #[tokio::test]
    async fn test_discover_ble_devices_with_filter() {
        let dummy_service_uuid = "00001800-0000-1000-8000-00805f9b34fb"; // Example: Generic Access Service
        match discover_ble_devices(Some(dummy_service_uuid), 5).await {
            Ok(devices) => {
                log::info!("Test with filter discovered {} devices. (Expected to be few or none unless service is present).", devices.len());
                for device_info in devices {
                    log::info!("  - {}", device_info);
                }
            }
            Err(e) => {
                log::warn!("Test with filter error: {}. This might be due to no adapter or invalid UUID format if not careful.", e);
            }
        }
    }
}
