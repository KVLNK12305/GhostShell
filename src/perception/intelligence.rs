//! Threat Intelligence - Ghost knows the enemy
//!
//! Provides:
//! - IOC matching
//! - Threat actor profiles
//! - Attack pattern recognition
//! - Intelligence feeds

use anyhow::Result;
use std::collections::{HashMap, HashSet};
use crate::perception::network::SocketRecord;
use crate::perception::system::ProcessInfo;
use crate::perception::auth::AuthEvent;
use crate::perception::{Threat, ThreatSeverity, Evidence, EvidenceKind};

/// Threat intelligence engine
pub struct ThreatIntelligence {
    iocs: HashSet<IOC>,
    threat_actors: HashMap<String, ThreatActor>,
    attack_patterns: Vec<AttackPattern>,
    confidence_threshold: f64,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct IOC {
    pub indicator: String,
    pub ioc_type: IOCType,
    pub description: String,
    pub severity: ThreatSeverity,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum IOCType {
    IP,
    Domain,
    Hash,
    FilePath,
    Process,
    Port,
}

#[derive(Debug, Clone)]
pub struct ThreatActor {
    pub name: String,
    pub aliases: Vec<String>,
    pub motives: Vec<String>,
    pub capabilities: Vec<String>,
    pub iocs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AttackPattern {
    pub name: String,
    pub description: String,
    pub indicators: Vec<String>,
    pub severity: ThreatSeverity,
}

impl ThreatIntelligence {
    pub fn new() -> Self {
        let mut engine = Self {
            iocs: HashSet::new(),
            threat_actors: HashMap::new(),
            attack_patterns: Vec::new(),
            confidence_threshold: 0.7,
        };

        // Load default IOCs
        engine.load_default_iocs();
        engine.load_default_threat_actors();

        engine
    }

    /// Load default IOCs
    fn load_default_iocs(&mut self) {
        // Malicious IPs (example)
        self.iocs.insert(IOC {
            indicator: "45.33.22.11".to_string(),
            ioc_type: IOCType::IP,
            description: "Known C2 server".to_string(),
            severity: ThreatSeverity::High,
        });

        self.iocs.insert(IOC {
            indicator: "malicious.xyz".to_string(),
            ioc_type: IOCType::Domain,
            description: "Malware distribution domain".to_string(),
            severity: ThreatSeverity::High,
        });

        // Suspicious ports
        self.iocs.insert(IOC {
            indicator: "4444".to_string(),
            ioc_type: IOCType::Port,
            description: "Common C2 beacon port".to_string(),
            severity: ThreatSeverity::Medium,
        });

        // Suspicious processes
        self.iocs.insert(IOC {
            indicator: "xmrig".to_string(),
            ioc_type: IOCType::Process,
            description: "Cryptominer".to_string(),
            severity: ThreatSeverity::High,
        });
    }

    /// Load default threat actors
    fn load_default_threat_actors(&mut self) {
        self.threat_actors.insert("SilverFox".to_string(), ThreatActor {
            name: "SilverFox".to_string(),
            aliases: vec!["APT28".to_string(), "Sofacy".to_string()],
            motives: vec!["Espionage".to_string(), "Financial".to_string()],
            capabilities: vec!["Ransomware".to_string(), "Data exfiltration".to_string()],
            iocs: vec!["45.33.22.11".to_string(), "malicious.xyz".to_string()],
        });
    }

    /// Check against threat intelligence
    pub async fn check(
        &self,
        sockets: &[SocketRecord],
        processes: &[ProcessInfo],
        auth: &[AuthEvent],
    ) -> Result<Vec<Threat>> {
        let mut threats = Vec::new();

        // Check socket connections against IOCs
        for socket in sockets {
            if let Some(threat) = self.check_socket(socket) {
                threats.push(threat);
            }
        }

        // Check processes against IOCs
        for process in processes {
            if let Some(threat) = self.check_process(process) {
                threats.push(threat);
            }
        }

        // Check auth events against IOCs
        for event in auth {
            if let Some(threat) = self.check_auth(event) {
                threats.push(threat);
            }
        }

        // Check attack patterns
        threats.extend(self.check_attack_patterns(sockets, processes, auth));

        Ok(threats)
    }

    /// Check socket against IOCs
    fn check_socket(&self, socket: &SocketRecord) -> Option<Threat> {
        let mut evidence = Vec::new();

        // Check IP
        if let Some(ioc) = self.iocs.iter().find(|i| i.indicator == socket.peer_addr) {
            evidence.push(Evidence {
                kind: EvidenceKind::SocketConnection,
                data: format!("Matching IOC: {}", ioc.indicator),
                timestamp: chrono::Utc::now(),
            });

            return Some(Threat {
                id: uuid::Uuid::new_v4().to_string(),
                severity: ioc.severity,
                source: "threat_intelligence".to_string(),
                description: format!("IOC match: {} - {}", ioc.indicator, ioc.description),
                timestamp: chrono::Utc::now(),
                evidence,
                confidence: 0.9,
            });
        }

        // Check port
        if let Some(port) = socket.peer_port {
            if let Some(ioc) = self.iocs.iter().find(|i| i.indicator == port.to_string()) {
                evidence.push(Evidence {
                    kind: EvidenceKind::SocketConnection,
                    data: format!("Port IOC match: {}", port),
                    timestamp: chrono::Utc::now(),
                });

                return Some(Threat {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: ioc.severity,
                    source: "threat_intelligence".to_string(),
                    description: format!("Suspicious port: {} - {}", port, ioc.description),
                    timestamp: chrono::Utc::now(),
                    evidence,
                    confidence: 0.8,
                });
            }
        }

        None
    }

    /// Check process against IOCs
    fn check_process(&self, process: &ProcessInfo) -> Option<Threat> {
        for ioc in self.iocs.iter().filter(|i| i.ioc_type == IOCType::Process) {
            if process.name.to_lowercase().contains(&ioc.indicator.to_lowercase()) {
                let evidence = vec![Evidence {
                    kind: EvidenceKind::Process,
                    data: format!("Process matches IOC: {} (PID: {})", process.name, process.pid),
                    timestamp: chrono::Utc::now(),
                }];

                return Some(Threat {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: ioc.severity,
                    source: "threat_intelligence".to_string(),
                    description: format!("Suspicious process: {} - {}", process.name, ioc.description),
                    timestamp: chrono::Utc::now(),
                    evidence,
                    confidence: 0.85,
                });
            }
        }

        None
    }

    /// Check auth event against IOCs
    fn check_auth(&self, event: &AuthEvent) -> Option<Threat> {
        if let Some(ip) = &event.source_ip {
            if let Some(ioc) = self.iocs.iter().find(|i| i.indicator == *ip) {
                let evidence = vec![Evidence {
                    kind: EvidenceKind::AuthEvent,
                    data: format!("Auth from suspicious IP: {}", ip),
                    timestamp: chrono::Utc::now(),
                }];

                return Some(Threat {
                    id: uuid::Uuid::new_v4().to_string(),
                    severity: ioc.severity,
                    source: "threat_intelligence".to_string(),
                    description: format!("Auth from IOC IP: {} - {}", ip, ioc.description),
                    timestamp: chrono::Utc::now(),
                    evidence,
                    confidence: 0.9,
                });
            }
        }

        None
    }

    /// Check attack patterns
    fn check_attack_patterns(
        &self,
        sockets: &[SocketRecord],
        processes: &[ProcessInfo],
        auth: &[AuthEvent],
    ) -> Vec<Threat> {
        let mut threats = Vec::new();

        // Pattern: Port scanning
        let scan_count = sockets.iter()
            .filter(|s| s.state.eq_ignore_ascii_case("SYN_RECV"))
            .count();
        
        if scan_count > 20 {
            threats.push(Threat {
                id: uuid::Uuid::new_v4().to_string(),
                severity: ThreatSeverity::High,
                source: "attack_patterns".to_string(),
                description: format!("Port scanning pattern: {} SYN_RECV connections", scan_count),
                timestamp: chrono::Utc::now(),
                evidence: vec![Evidence {
                    kind: EvidenceKind::NetworkTraffic,
                    data: "Excessive SYN_RECV connections".to_string(),
                    timestamp: chrono::Utc::now(),
                }],
                confidence: 0.85,
            });
        }

        // Pattern: Brute force
        let failed_count = auth.iter()
            .filter(|a| !a.success)
            .count();
        
        if failed_count > 10 {
            threats.push(Threat {
                id: uuid::Uuid::new_v4().to_string(),
                severity: ThreatSeverity::High,
                source: "attack_patterns".to_string(),
                description: format!("Brute force pattern: {} failed auth attempts", failed_count),
                timestamp: chrono::Utc::now(),
                evidence: vec![Evidence {
                    kind: EvidenceKind::AuthEvent,
                    data: "Excessive failed authentications".to_string(),
                    timestamp: chrono::Utc::now(),
                }],
                confidence: 0.9,
            });
        }

        // Pattern: C2 beacon detection
        let beacon_count = sockets.iter()
            .filter(|s| {
                if let Some(port) = s.peer_port {
                    port == 4444 || port == 5555 || port == 8443
                } else {
                    false
                }
            })
            .count();
        
        if beacon_count > 0 {
            threats.push(Threat {
                id: uuid::Uuid::new_v4().to_string(),
                severity: ThreatSeverity::Critical,
                source: "attack_patterns".to_string(),
                description: format!("C2 beacon pattern: {} connections to known C2 ports", beacon_count),
                timestamp: chrono::Utc::now(),
                evidence: vec![Evidence {
                    kind: EvidenceKind::NetworkTraffic,
                    data: "Connections to known C2 ports".to_string(),
                    timestamp: chrono::Utc::now(),
                }],
                confidence: 0.95,
            });
        }

        threats
    }

    /// Add IOC
    pub fn add_ioc(&mut self, ioc: IOC) {
        self.iocs.insert(ioc);
    }

    /// Remove IOC
    pub fn remove_ioc(&mut self, indicator: &str) {
        self.iocs.retain(|i| i.indicator != indicator);
    }

    /// Get all IOCs
    pub fn get_iocs(&self) -> &HashSet<IOC> {
        &self.iocs
    }
}