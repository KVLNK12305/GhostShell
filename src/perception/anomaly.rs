//! Anomaly Detection - Ghost finds what's wrong
//!
//! Detects:
//! - Context-aware anomalies
//! - Attack patterns
//! - Baseline deviations
//! - Suspicious behavior

use std::collections::{HashMap, HashSet};
use crate::perception::network::SocketRecord;
use crate::perception::system::ProcessInfo;
use crate::perception::auth::AuthEvent;

/// Anomaly severity
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Info,
    Suspicious,
    Critical,
}

/// Anomaly detected
#[derive(Debug, Clone)]
pub struct Anomaly {
    pub severity: Severity,
    pub text: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Anomaly detector
pub struct AnomalyDetector {
    context_profiles: Vec<ProjectProfile>,
    active_contexts: HashSet<String>,
    tracked_ports: HashSet<u16>,
    baseline: Baseline,
}

#[derive(Debug, Clone)]
struct ProjectProfile {
    name: String,
    keywords: Vec<String>,
    ports: Vec<u16>,
}

#[derive(Debug, Clone)]
struct Baseline {
    connections: HashMap<String, usize>,
    processes: HashMap<String, usize>,
    ports: HashMap<u16, usize>,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            context_profiles: vec![
                ProjectProfile {
                    name: "Project Mercury".to_string(),
                    keywords: vec!["mercury".to_string(), "mercury-api".to_string()],
                    ports: vec![8443, 9000, 50051],
                },
                ProjectProfile {
                    name: "Project Atlas".to_string(),
                    keywords: vec!["atlas".to_string(), "atlas-edge".to_string()],
                    ports: vec![443, 9443, 8080],
                },
                ProjectProfile {
                    name: "Project Orion".to_string(),
                    keywords: vec!["orion".to_string(), "orion-relay".to_string()],
                    ports: vec![22, 2222, 3389],
                },
            ],
            active_contexts: HashSet::new(),
            tracked_ports: HashSet::new(),
            baseline: Baseline {
                connections: HashMap::new(),
                processes: HashMap::new(),
                ports: HashMap::new(),
            },
        };

        // Initialize tracked ports
        for profile in &detector.context_profiles {
            for port in &profile.ports {
                *detector.baseline.ports.entry(*port).or_insert(0) += 1;
            }
        }

        detector
    }

    /// Detect anomalies
    pub async fn detect(
        &mut self,
        sockets: &[SocketRecord],
        processes: &[ProcessInfo],
        auth: &[AuthEvent],
    ) -> Result<Vec<Anomaly>, anyhow::Error> {
        let mut anomalies = Vec::new();

        // Update context
        self.update_context();

        // Detect context-based anomalies
        anomalies.extend(self.detect_context_anomalies(sockets));

        // Detect attack patterns
        anomalies.extend(self.detect_attack_patterns(sockets, processes, auth));

        // Detect baseline deviations
        anomalies.extend(self.detect_baseline_deviations(sockets));

        // Detect process anomalies
        anomalies.extend(self.detect_process_anomalies(processes));

        Ok(anomalies)
    }

    /// Update context from active profiles
    fn update_context(&mut self) {
        // Use all profiles for now
        for profile in &self.context_profiles {
            self.active_contexts.insert(profile.name.clone());
            for port in &profile.ports {
                self.tracked_ports.insert(*port);
            }
        }
    }

    /// Detect context-based anomalies
    fn detect_context_anomalies(&self, sockets: &[SocketRecord]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        for rec in sockets {
            let Some(local_port) = rec.local_port else { continue };

            if !self.tracked_ports.contains(&local_port) {
                continue;
            }

            let listen_state = rec.state.eq_ignore_ascii_case("LISTEN")
                || rec.state.eq_ignore_ascii_case("UNCONN");

            if listen_state && self.is_exposed_addr(&rec.local_addr) {
                anomalies.push(Anomaly {
                    severity: Severity::Critical,
                    text: format!(
                        "Context port {} exposed on {} ({})",
                        local_port,
                        rec.local_addr,
                        self.process_label(rec)
                    ),
                    timestamp: chrono::Utc::now(),
                });
                continue;
            }

            if let Some(peer_port) = rec.peer_port {
                if !self.is_internal_addr(&rec.peer_addr) && peer_port != 0 {
                    anomalies.push(Anomaly {
                        severity: Severity::Suspicious,
                        text: format!(
                            "Context port {} talks to external {}:{} ({})",
                            local_port,
                            rec.peer_addr,
                            peer_port,
                            self.process_label(rec)
                        ),
                        timestamp: chrono::Utc::now(),
                    });
                    continue;
                }
            }

            anomalies.push(Anomaly {
                severity: Severity::Info,
                text: format!(
                    "Context port {} observed in {} state ({})",
                    local_port,
                    rec.state,
                    self.process_label(rec)
                ),
                timestamp: chrono::Utc::now(),
            });
        }

        anomalies
    }

    /// Detect attack patterns
    fn detect_attack_patterns(&self, sockets: &[SocketRecord], processes: &[ProcessInfo], auth: &[AuthEvent]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // 1. Port scanning
        let scan_count = sockets.iter()
            .filter(|s| s.state.eq_ignore_ascii_case("SYN_RECV"))
            .count();
        
        if scan_count > 10 {
            anomalies.push(Anomaly {
                severity: Severity::Critical,
                text: format!("Port scanning detected: {} SYN_RECV connections", scan_count),
                timestamp: chrono::Utc::now(),
            });
        }

        // 2. Auth brute force
        let failed_count = auth.iter()
            .filter(|a| !a.success)
            .count();
        
        if failed_count > 5 {
            anomalies.push(Anomaly {
                severity: Severity::Suspicious,
                text: format!("Auth brute force: {} failed attempts", failed_count),
                timestamp: chrono::Utc::now(),
            });
        }

        // 3. Multiple failed attempts from same IP
        let mut ip_counts = HashMap::new();
        for event in auth {
            if let Some(ip) = &event.source_ip {
                if !event.success {
                    *ip_counts.entry(ip.clone()).or_insert(0) += 1;
                }
            }
        }
        for (ip, count) in ip_counts {
            if count > 3 {
                anomalies.push(Anomaly {
                    severity: Severity::Suspicious,
                    text: format!("Multiple failed attempts from {}: {} attempts", ip, count),
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        // 4. Suspicious processes
        for process in processes {
            if process.name.contains("cmd") || process.name.contains("powershell") {
                if let Some(parent) = &process.parent_name {
                    if parent.contains("explorer") || parent.contains("svchost") {
                        anomalies.push(Anomaly {
                            severity: Severity::High,
                            text: format!("Suspicious process: {} (PID: {}) from {}", 
                                process.name, process.pid, parent),
                            timestamp: chrono::Utc::now(),
                        });
                    }
                }
            }
        }

        anomalies
    }

    /// Detect baseline deviations
    fn detect_baseline_deviations(&self, sockets: &[SocketRecord]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        let mut port_counts: HashMap<u16, usize> = HashMap::new();

        for rec in sockets {
            if let Some(port) = rec.local_port {
                *port_counts.entry(port).or_insert(0) += 1;
            }
        }

        for (port, count) in port_counts {
            if let Some(baseline) = self.baseline.ports.get(&port) {
                if count > baseline * 3 {
                    anomalies.push(Anomaly {
                        severity: Severity::Suspicious,
                        text: format!("Unusual port activity: port {} has {} connections (baseline: {})", 
                            port, count, baseline),
                        timestamp: chrono::Utc::now(),
                    });
                }
            }
        }

        anomalies
    }

    /// Detect process anomalies
    fn detect_process_anomalies(&self, processes: &[ProcessInfo]) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        for process in processes {
            // Check for hidden processes
            if process.name.starts_with(".") && process.name.len() < 10 {
                anomalies.push(Anomaly {
                    severity: Severity::Suspicious,
                    text: format!("Hidden process detected: {} (PID: {})", process.name, process.pid),
                    timestamp: chrono::Utc::now(),
                });
            }

            // Check for root/administrator processes
            if process.user == "root" || process.user == "Administrator" {
                if process.name.contains("http") || process.name.contains("ssh") {
                    tracing::debug!("Root-owned network service: {} (PID: {})", process.name, process.pid);
                }
            }
        }

        anomalies
    }

    /// Helper: Check if address is exposed
    fn is_exposed_addr(&self, addr: &str) -> bool {
        let stripped = addr.trim_matches('[').trim_matches(']');
        stripped == "0.0.0.0" || stripped == "::" || stripped == "*"
    }

    /// Helper: Check if address is internal
    fn is_internal_addr(&self, addr: &str) -> bool {
        let stripped = addr.trim_matches('[').trim_matches(']');
        if stripped == "*" || stripped == "0.0.0.0" || stripped == "::" {
            return true;
        }
        if stripped.starts_with("127.") || stripped.starts_with("10.") || stripped.starts_with("192.168.") {
            return true;
        }
        if stripped == "localhost" || stripped == "::1" {
            return true;
        }
        false
    }

    /// Helper: Get process label
    fn process_label(&self, rec: &SocketRecord) -> String {
        rec.process
            .as_ref()
            .map_or_else(|| "unknown-process".to_string(), |s| s.clone())
    }

    /// Get tracked ports
    pub fn get_tracked_ports(&self) -> &HashSet<u16> {
        &self.tracked_ports
    }

    /// Get active contexts
    pub fn get_active_contexts(&self) -> &HashSet<String> {
        &self.active_contexts
    }
}