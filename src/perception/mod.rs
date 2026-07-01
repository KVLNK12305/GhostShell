//! Perception Engine - Ghost's Senses
//!
//! This module contains all of Ghost's perception capabilities:
//! - Network monitoring
//! - System monitoring
//! - Auth log monitoring
//! - Anomaly detection
//! - Threat intelligence

pub mod network;
pub mod system;
pub mod auth;
pub mod anomaly;
pub mod intelligence;

use anyhow::Result;
use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub use network::NetworkMonitor;
pub use system::SystemMonitor;
pub use auth::AuthMonitor;
pub use anomaly::AnomalyDetector;
pub use intelligence::ThreatIntelligence;

/// Threat detected by Ghost
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    pub id: String,
    pub severity: ThreatSeverity,
    pub source: String,
    pub description: String,
    pub timestamp: DateTime<Utc>,
    pub evidence: Vec<Evidence>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ThreatSeverity {
    Critical = 4,
    High = 3,
    Medium = 2,
    Low = 1,
    Info = 0,
}

impl std::fmt::Display for ThreatSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatSeverity::Critical => write!(f, "CRITICAL"),
            ThreatSeverity::High => write!(f, "HIGH"),
            ThreatSeverity::Medium => write!(f, "MEDIUM"),
            ThreatSeverity::Low => write!(f, "LOW"),
            ThreatSeverity::Info => write!(f, "INFO"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub kind: EvidenceKind,
    pub data: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceKind {
    SocketConnection,
    Process,
    AuthEvent,
    FileChange,
    NetworkTraffic,
    SystemLog,
    UserBehavior,
}

/// Main perception engine
pub struct PerceptionEngine {
    network: NetworkMonitor,
    system: SystemMonitor,
    auth: AuthMonitor,
    anomaly: AnomalyDetector,
    intelligence: ThreatIntelligence,
    threat_history: VecDeque<Threat>,
    max_history: usize,
}

impl PerceptionEngine {
    pub fn new() -> Self {
        Self {
            network: NetworkMonitor::new(),
            system: SystemMonitor::new(),
            auth: AuthMonitor::new(),
            anomaly: AnomalyDetector::new(),
            intelligence: ThreatIntelligence::new(),
            threat_history: VecDeque::with_capacity(1000),
            max_history: 1000,
        }
    }

    /// Start all monitors
    pub async fn start_monitoring(&mut self) -> Result<()> {
        tracing::info!("Starting perception engines...");
        
        self.network.start().await?;
        self.system.start().await?;
        self.auth.start().await?;
        
        tracing::info!("All perception engines active");
        Ok(())
    }

    /// Scan for threats
    pub async fn scan(&mut self) -> Result<Vec<Threat>> {
        let mut threats = Vec::new();

        // Gather intelligence from all sources
        let sockets = self.network.get_sockets().await?;
        let processes = self.system.get_processes().await?;
        let auth_events = self.auth.get_events().await?;
        
        // Detect anomalies
        let anomalies = self.anomaly.detect(&sockets, &processes, &auth_events).await?;
        
        // Correlate threats
        for anomaly in anomalies {
            if let Some(threat) = self.correlate_threat(anomaly, &sockets, &processes, &auth_events).await {
                threats.push(threat);
            }
        }

        // Check threat intelligence
        let intel_threats = self.intelligence.check(&sockets, &processes, &auth_events).await?;
        threats.extend(intel_threats);

        // Store threat history
        for threat in &threats {
            self.threat_history.push_back(threat.clone());
            if self.threat_history.len() > self.max_history {
                self.threat_history.pop_front();
            }
        }

        Ok(threats)
    }

    /// Correlate an anomaly into a threat
    async fn correlate_threat(
        &self,
        anomaly: anomaly::Anomaly,
        sockets: &[network::SocketRecord],
        _processes: &[system::ProcessInfo],
        _auth_events: &[auth::AuthEvent],
    ) -> Option<Threat> {
        let severity = match anomaly.severity {
            anomaly::Severity::Critical => ThreatSeverity::Critical,
            anomaly::Severity::Suspicious => ThreatSeverity::High,
            anomaly::Severity::Info => ThreatSeverity::Low,
        };

        let mut evidence = Vec::new();
        evidence.push(Evidence {
            kind: EvidenceKind::SystemLog,
            data: anomaly.text.clone(),
            timestamp: chrono::Utc::now(),
        });

        // Find related evidence
        for socket in sockets {
            if let Some(port) = socket.local_port {
                if anomaly.text.contains(&port.to_string()) {
                    evidence.push(Evidence {
                        kind: EvidenceKind::SocketConnection,
                        data: format!("{}:{} -> {}:{}", socket.protocol, port, socket.peer_addr, socket.peer_port.unwrap_or(0)),
                        timestamp: chrono::Utc::now(),
                    });
                }
            }
        }

        Some(Threat {
            id: uuid::Uuid::new_v4().to_string(),
            severity,
            source: "anomaly_detector".to_string(),
            description: anomaly.text,
            timestamp: chrono::Utc::now(),
            evidence,
            confidence: 0.8,
        })
    }

    /// Get threat history
    pub fn get_history(&self) -> &VecDeque<Threat> {
        &self.threat_history
    }

    /// Clear threat history
    pub fn clear_history(&mut self) {
        self.threat_history.clear();
    }
}