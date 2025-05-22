use std::time::Duration;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use crate::ErrorCode;
use log;
pub fn discovery(service: &str, seconds:u64) -> Result<Vec<ServiceInfo>, ErrorCode>{
    let mdns = ServiceDaemon::new().map_err(|e| {
        log::warn!("new ServiceDaemon error {}", e);
        ErrorCode::MDNSInitFailure
    })?;
    let receiver  = mdns.browse(service).map_err(|e| {
        log::warn!("mdns browser error {}", e);
        ErrorCode::MDNSInitFailure
    })?;

    let now = std::time::Instant::now();
    let mut r = Vec::new();

    while let Ok(event) = receiver.recv_timeout(Duration::from_secs(seconds)) {
        match event {
            ServiceEvent::ServiceResolved(info) => {
                log::debug!(
                    "At {:?}: Resolved a new service: {}\n host: {}\n port: {}",
                    now.elapsed(),
                    info.get_fullname(),
                    info.get_hostname(),
                    info.get_port(),
                );
                for addr in info.get_addresses().iter() {
                    log::debug!(" Address: {}", addr);
                }
                for prop in info.get_properties().iter() {
                    log::debug!(" Property: {}", prop);
                }
                r.push(info);
            }
            other_event => {
                log::debug!("At {:?}: {:?}", now.elapsed(), &other_event);
            }
        }
    }
    


    Ok(r)

}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use mdns_sd::{ServiceDaemon, ServiceInfo};

    #[test]
    pub fn test_discovery() {
        // With `enable_addr_auto()`, we can give empty addrs and let the lib find them.
        // If the caller knows specific addrs to use, then assign the addrs here.
        let my_addrs = "";
        let service_type = "_tiny-protocol._tcp.local.";
        let instance_name = "instance";
        let service_hostname = "tiny-protocol-hostname.local.";
        let port = 3456;
        let long = "two".repeat(30);
        let properties = [("Path", long.as_str()), ("Pa1", "three"), ("PATH", "one")/*one could not be found*/,];

        let service_info = ServiceInfo::new(
            &service_type,
            instance_name,
            &service_hostname,
            my_addrs,
            port,
            &properties[..],
        ).expect("valid service info")
            .enable_addr_auto();
        let mdns = ServiceDaemon::new().expect("service init");
        // let monitor = mdns.monitor().expect("Failed to monitor the daemon");
        let service_fullname = service_info.get_fullname().to_string();
        mdns.register(service_info)
            .expect("Failed to register mDNS service");
        println!("Registered service {}.{}", &instance_name, &service_type);
        std::thread::spawn(move || {
            let wait_in_secs = 3;
            println!("Sleeping {} seconds before unregister", wait_in_secs);
            std::thread::sleep(Duration::from_secs(wait_in_secs));

            let receiver = mdns.unregister(&service_fullname).unwrap();
            while let Ok(event) = receiver.recv() {
                println!("unregister result: {:?}", &event);
            }
        });
        super::discovery(service_type, 5).unwrap();
        println!("finish");
    }
}
