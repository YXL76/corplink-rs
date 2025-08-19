use std::collections::HashMap;
use std::io::Error;
use std::process::Command;

const SYSTEMD_RESOLVED_CONF: &str = "/etc/systemd/resolved.conf";
const SYSTEMD_RESOLVED_CONF_BACKUP: &str = "/etc/systemd/resolved.conf.corplink.bak";

pub struct DNSManager {
    service_dns: HashMap<String, String>,
    service_dns_search: HashMap<String, String>,
}

impl DNSManager {
    pub fn new() -> DNSManager {
        DNSManager {
            service_dns: HashMap::new(),
            service_dns_search: HashMap::new(),
        }
    }

    pub fn set_dns(&mut self, dns_servers: Vec<&str>, dns_search: Vec<&str>) -> Result<(), Error> {
        if dns_servers.is_empty() {
            return Ok(());
        }
        Command::new("cp")
            .arg(SYSTEMD_RESOLVED_CONF)
            .arg(SYSTEMD_RESOLVED_CONF_BACKUP)
            .output()?;
        std::fs::write(
            SYSTEMD_RESOLVED_CONF,
            format!("[Resolve]\nDNS={}\n", dns_servers.join(" ")),
        )?;
        if !dns_search.is_empty() {
            std::fs::write(
                SYSTEMD_RESOLVED_CONF,
                format!("Domains={}\n", dns_search.join(" ")),
            )?;
        }
        Command::new("systemctl")
            .arg("restart")
            .arg("systemd-resolved")
            .output()?;

        Ok(())
    }

    pub fn restore_dns(&self) -> Result<(), Error> {
        Command::new("cp")
            .arg(SYSTEMD_RESOLVED_CONF_BACKUP)
            .arg(SYSTEMD_RESOLVED_CONF)
            .output()?;
        Command::new("systemctl")
            .arg("restart")
            .arg("systemd-resolved")
            .output()?;
        Ok(())
    }
}
