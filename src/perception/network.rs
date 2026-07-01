//! Network Perception - Ghost sees network activity
//!
//! Monitors:
//! - Socket connections
//! - Port activity
//! - Network traffic patterns
//! - Service fingerprints

use anyhow::Result;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::timeout;

/// Socket connection record
#[derive(Debug, Clone)]
pub struct SocketRecord {
    pub protocol: String,
    pub state: String,
    pub local_addr: String,
    pub local_port: Option<u16>,
    pub peer_addr: String,
    pub peer_port: Option<u16>,
    pub process: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Network monitor
pub struct NetworkMonitor {
    history: VecDeque<SocketRecord>,
    suspicious: Vec<SocketRecord>,
    max_history: usize,
    scan_interval: Duration,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            history: VecDeque::with_capacity(10000),
            suspicious: Vec::new(),
            max_history: 10000,
            scan_interval: Duration::from_secs(5),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Network monitor active");
        Ok(())
    }

    /// Get current socket snapshot
    pub async fn get_sockets(&mut self) -> Result<Vec<SocketRecord>> {
        let output = self.get_socket_output().await?;
        let records = self.parse_socket_output(&output);
        
        // Update history
        for mut record in records.clone() {
            record.timestamp = chrono::Utc::now();
            self.history.push_back(record);
            if self.history.len() > self.max_history {
                self.history.pop_front();
            }
        }

        // Detect anomalies
        self.detect_anomalies(&records);

        Ok(records)
    }

    /// Get raw socket output
    async fn get_socket_output(&self) -> Result<String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("netstat")
                .args(["-ano"])
                .output()
                .await?
        } else {
            // Try ss first, fallback to netstat
            let ss_output = Command::new("sh")
                .arg("-c")
                .arg("ss -tulnp 2>/dev/null")
                .output()
                .await?;
            
            if !ss_output.stdout.is_empty() {
                ss_output
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg("netstat -tulnp 2>/dev/null")
                    .output()
                    .await?
            }
        };

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Parse socket output into records
    fn parse_socket_output(&self, output: &str) -> Vec<SocketRecord> {
        let mut records = Vec::new();

        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Skip headers
            if trimmed.starts_with("Netid") || trimmed.starts_with("Proto") {
                continue;
            }
            if trimmed.contains("State") || trimmed.contains("Local Address") {
                continue;
            }

            // Parse ss format
            if let Some(record) = self.parse_ss_line(trimmed) {
                records.push(record);
                continue;
            }

            // Parse netstat format
            if let Some(record) = self.parse_netstat_line(trimmed) {
                records.push(record);
            }
        }

        records
    }

    /// Parse ss (socket statistics) line
    fn parse_ss_line(&self, line: &str) -> Option<SocketRecord> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }

        let protocol = parts[0].to_string();
        let state = parts[1].to_string();
        let local = parts[4];
        let peer = parts[5];
        let process = if parts.len() > 6 {
            Some(parts[6..].join(" "))
        } else {
            None
        };

        let (local_addr, local_port) = self.parse_endpoint(local);
        let (peer_addr, peer_port) = self.parse_endpoint(peer);

        Some(SocketRecord {
            protocol,
            state,
            local_addr,
            local_port,
            peer_addr,
            peer_port,
            process,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Parse netstat line
    fn parse_netstat_line(&self, line: &str) -> Option<SocketRecord> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            return None;
        }

        let protocol = parts[0].to_string();
        let local = parts[3];
        let peer = parts[4];
        let state = if parts.len() > 5 {
            parts[5].to_string()
        } else {
            "UNKNOWN".to_string()
        };
        let process = if parts.len() > 6 {
            Some(parts[6..].join(" "))
        } else {
            None
        };

        let (local_addr, local_port) = self.parse_endpoint(local);
        let (peer_addr, peer_port) = self.parse_endpoint(peer);

        Some(SocketRecord {
            protocol,
            state,
            local_addr,
            local_port,
            peer_addr,
            peer_port,
            process,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Parse endpoint (address:port)
    fn parse_endpoint(&self, endpoint: &str) -> (String, Option<u16>) {
        let cleaned = endpoint.trim().trim_matches('[').trim_matches(']');
        if cleaned.is_empty() || cleaned == "*" {
            return ("*".to_string(), None);
        }

        if let Some((addr, port_str)) = cleaned.rsplit_once(':') {
            let port = port_str.parse::<u16>().ok();
            return (addr.to_string(), port);
        }

        (cleaned.to_string(), None)
    }

    /// Detect network anomalies
    fn detect_anomalies(&mut self, records: &[SocketRecord]) {
        // Port scanning detection
        let syn_count = records.iter()
            .filter(|r| r.state.eq_ignore_ascii_case("SYN_RECV"))
            .count();
        
        if syn_count > 20 {
            tracing::warn!("Possible port scan detected: {} SYN_RECV connections", syn_count);
        }

        // C2 beacon detection
        for record in records {
            if let Some(peer_port) = record.peer_port {
                let c2_ports = [4444, 5555, 6666, 8443, 1337, 8080];
                if c2_ports.contains(&peer_port) {
                    self.suspicious.push(record.clone());
                    tracing::warn!("Possible C2 beacon: {} -> {}:{}", 
                        record.local_addr, record.peer_addr, peer_port);
                }
            }
        }

        // Data exfiltration detection
        for record in records {
            if let Some(local_port) = record.local_port {
                if local_port > 1024 && record.protocol == "tcp" {
                    if let Some(peer_port) = record.peer_port {
                        if peer_port == 443 && !record.peer_addr.starts_with("10.") {
                            tracing::debug!("Possible exfiltration from port {}", local_port);
                        }
                    }
                }
            }
        }
    }

    /// Get suspicious connections
    pub fn get_suspicious(&self) -> &[SocketRecord] {
        &self.suspicious
    }

    /// Clear suspicious list
    pub fn clear_suspicious(&mut self) {
        self.suspicious.clear();
    }
}