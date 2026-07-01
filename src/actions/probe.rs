//! Probing Engine - Active Intelligence Gathering
//!
//! Ghost probes threats to gather intelligence:
//! - Port scanning
//! - Service fingerprinting
//! - Vulnerability scanning
//! - Network mapping

use anyhow::Result;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::perception::Threat;
use crate::actions::ActionResult;

/// Probing engine for active intelligence gathering
pub struct ProbeEngine {
    scan_timeout: Duration,
    max_parallel: usize,
    stealth_mode: bool,
    fingerprints: Vec<String>,
}

impl ProbeEngine {
    pub fn new() -> Self {
        Self {
            scan_timeout: Duration::from_secs(5),
            max_parallel: 10,
            stealth_mode: true,
            fingerprints: Vec::new(),
        }
    }

    /// Execute probe against a threat
    pub async fn execute(&self, threat: &Threat) -> Result<ActionResult> {
        tracing::debug!("Probing threat: {}", threat.id);
        
        let mut evidence = Vec::new();
        let mut success = true;
        let mut message = String::new();

        // Extract target from threat evidence
        let targets = self.extract_targets(threat);
        
        if targets.is_empty() {
            return Ok(ActionResult {
                action: crate::actions::Action::Probe(threat.clone()),
                success: false,
                message: "No targets found in threat evidence".to_string(),
                timestamp: chrono::Utc::now(),
                evidence: vec![],
            });
        }

        // Probe each target
        for target in targets {
            match self.probe_target(&target).await {
                Ok(results) => {
                    evidence.extend(results);
                    message.push_str(&format!("Probed {}: success\n", target));
                }
                Err(e) => {
                    success = false;
                    message.push_str(&format!("Probed {}: failed - {}\n", target, e));
                }
            }
        }

        Ok(ActionResult {
            action: crate::actions::Action::Probe(threat.clone()),
            success,
            message,
            timestamp: chrono::Utc::now(),
            evidence,
        })
    }

    /// Probe a single target
    async fn probe_target(&self, target: &str) -> Result<Vec<String>> {
        let mut results = Vec::new();

        // Parse target
        if let Ok(ip) = target.parse::<IpAddr>() {
            // IP address - scan common ports
            let ports = [22, 23, 25, 53, 80, 443, 445, 3389, 8080, 8443];
            for port in ports {
                if let Ok(result) = self.scan_port(ip, port).await {
                    results.push(result);
                }
            }
        } else if let Ok(socket) = target.parse::<SocketAddr>() {
            // Socket address - probe specific port
            if let Ok(result) = self.probe_socket(socket).await {
                results.push(result);
            }
        } else {
            // Hostname - resolve and probe
            if let Ok(ips) = tokio::net::lookup_host(target).await {
                for ip in ips {
                    let port = 80; // Default HTTP
                    if let Ok(result) = self.scan_port(ip.ip(), port).await {
                        results.push(result);
                    }
                }
            }
        }

        Ok(results)
    }

    /// Scan a single port
    async fn scan_port(&self, ip: IpAddr, port: u16) -> Result<String> {
        let addr = SocketAddr::new(ip, port);
        
        match timeout(self.scan_timeout, TcpStream::connect(&addr)).await {
            Ok(Ok(_)) => {
                // Port is open - fingerprint service
                let fingerprint = self.fingerprint_service(ip, port).await?;
                Ok(format!("Port {} open on {} - {}", port, ip, fingerprint))
            }
            _ => {
                // Port is closed or filtered
                Ok(format!("Port {} closed/filtered on {}", port, ip))
            }
        }
    }

    /// Probe a specific socket
    async fn probe_socket(&self, socket: SocketAddr) -> Result<String> {
        match timeout(self.scan_timeout, TcpStream::connect(&socket)).await {
            Ok(Ok(_)) => {
                let fingerprint = self.fingerprint_service(socket.ip(), socket.port()).await?;
                Ok(format!("{}:{} - {}", socket.ip(), socket.port(), fingerprint))
            }
            _ => {
                Ok(format!("{}:{} - unreachable", socket.ip(), socket.port()))
            }
        }
    }

    /// Fingerprint service on port
    async fn fingerprint_service(&self, ip: IpAddr, port: u16) -> Result<String> {
        // Simple fingerprinting based on port
        let service = match port {
            22 => "SSH",
            23 => "Telnet",
            25 => "SMTP",
            53 => "DNS",
            80 => "HTTP",
            443 => "HTTPS",
            445 => "SMB",
            3389 => "RDP",
            8080 => "HTTP-Alt",
            8443 => "HTTPS-Alt",
            _ => "Unknown",
        };

        // Try to get more detailed fingerprint
        let detailed = self.get_detailed_fingerprint(ip, port).await;
        
        Ok(format!("{} ({})", service, detailed.unwrap_or_else(|| "unknown version".to_string())))
    }

    /// Get detailed service fingerprint
    async fn get_detailed_fingerprint(&self, ip: IpAddr, port: u16) -> Option<String> {
        let addr = SocketAddr::new(ip, port);
        
        // Attempt to read banner
        match timeout(Duration::from_secs(3), self.read_banner(addr)).await {
            Ok(Ok(banner)) => {
                // Parse banner for version info
                if banner.contains("SSH") {
                    return Some(self.parse_ssh_banner(&banner));
                } else if banner.contains("HTTP") {
                    return Some(self.parse_http_banner(&banner));
                }
                Some(banner.chars().take(50).collect())
            }
            _ => None,
        }
    }

    /// Read service banner
    async fn read_banner(&self, addr: SocketAddr) -> Result<String> {
        let mut stream = TcpStream::connect(addr).await?;
        let mut buffer = vec![0u8; 1024];
        let n = stream.try_read(&mut buffer)?;
        Ok(String::from_utf8_lossy(&buffer[..n]).to_string())
    }

    /// Parse SSH banner
    fn parse_ssh_banner(&self, banner: &str) -> String {
        // Extract SSH version
        if let Some(version) = banner.lines().next() {
            return version.to_string();
        }
        "SSH (unknown version)".to_string()
    }

    /// Parse HTTP banner
    fn parse_http_banner(&self, banner: &str) -> String {
        // Extract HTTP server header
        for line in banner.lines() {
            if line.to_lowercase().starts_with("server:") {
                return line.to_string();
            }
        }
        "HTTP (unknown server)".to_string()
    }

    /// Extract targets from threat evidence
    fn extract_targets(&self, threat: &Threat) -> Vec<String> {
        let mut targets = Vec::new();
        
        for evidence in &threat.evidence {
            // Extract IP addresses
            self.extract_ips(&evidence.data, &mut targets);
            
            // Extract hostnames
            self.extract_hostnames(&evidence.data, &mut targets);
        }
        
        targets
    }

    /// Extract IP addresses from text
    fn extract_ips(&self, text: &str, targets: &mut Vec<String>) {
        // Simple IP regex
        let re = regex::Regex::new(r"\b(\d{1,3}\.){3}\d{1,3}\b").unwrap();
        for cap in re.captures_iter(text) {
            if let Some(ip) = cap.get(0) {
                targets.push(ip.as_str().to_string());
            }
        }
    }

    /// Extract hostnames from text
    fn extract_hostnames(&self, text: &str, targets: &mut Vec<String>) {
        // Simple hostname regex
        let re = regex::Regex::new(r"\b([a-zA-Z0-9][a-zA-Z0-9\-]{1,61}[a-zA-Z0-9]\.[a-zA-Z]{2,})\b").unwrap();
        for cap in re.captures_iter(text) {
            if let Some(hostname) = cap.get(0) {
                targets.push(hostname.as_str().to_string());
            }
        }
    }
}

// Add regex dependency for extraction
// Add to Cargo.toml: regex = "1.10"