use pnet::datalink::{self, NetworkInterface};
use std::net::IpAddr;

pub fn scan_networks(interface: &str) -> Vec<IpAddr> {
    let interfaces = datalink::interfaces();
    let target_interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface)
        .expect("Interface not found");

    // Lógica de escaneo (ejemplo simplificado)
    vec![
        IpAddr::V4("192.168.1.1".parse().unwrap()),
        IpAddr::V4("192.168.1.2".parse().unwrap()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scan() {
        assert_eq!(scan_networks("wlan0").len(), 2);
    }
}
