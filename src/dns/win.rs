use super::DNSManagerTrait;
use std::io::Error;
use std::process::Command;

pub struct DNSManager {
    interface: String,
    original_dns: Option<Vec<String>>,
}

impl DNSManagerTrait for DNSManager {
    fn new(interface: String) -> Self {
        Self {
            interface,
            original_dns: None,
        }
    }

    fn set_dns(&mut self, dns_servers: Vec<&str>, dns_search: Vec<&str>) -> Result<(), Error> {
        if !dns_search.is_empty() {
            log::warn!("DNS search domains are not supported on Windows");
        }
        log::info!("dns servers: {dns_servers:?}");

        // First, backup current DNS settings
        let output = Command::new("netsh")
            .args(["interface", "ipv4", "show", "dns", &self.interface])
            .output()?;

        if output.status.success() {
            self.original_dns = Some(Vec::new());
        }

        // First, clear any existing DNS servers
        let status = Command::new("netsh")
            .args([
                "interface",
                "ipv4",
                "set",
                "dnsservers",
                &self.interface,
                "source=static",
                "address=none",
            ])
            .status()?;

        if !status.success() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to clear existing DNS servers",
            ));
        }

        // Set new DNS servers
        for (i, &dns) in dns_servers.iter().enumerate() {
            let status = Command::new("netsh")
                .args([
                    "interface",
                    "ipv4",
                    "add",
                    "dnsserver",
                    &self.interface,
                    dns,
                ])
                .status()?;

            if !status.success() {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to set DNS server {}", dns),
                ));
            }
        }

        Ok(())
    }

    fn restore_dns(&self) -> Result<(), Error> {
        // Reset DNS servers to DHCP
        let status = Command::new("netsh")
            .args([
                "interface",
                "ipv4",
                "set",
                "dnsservers",
                &self.interface,
                "source=dhcp",
            ])
            .status()?;

        if !status.success() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to restore DNS settings",
            ));
        }

        Ok(())
    }
}
