//! Authentication Perception - Ghost sees auth activity
//!
//! Monitors:
//! - Authentication attempts
//! - Failed logins
//! - Privilege escalations
//! - User sessions

use anyhow::Result;
use std::collections::VecDeque;
use chrono::{DateTime, Local, Utc};
use tokio::process::Command;

/// Authentication event
#[derive(Debug, Clone)]
pub struct AuthEvent {
    pub timestamp: DateTime<Utc>,
    pub username: String,
    pub source_ip: Option<String>,
    pub success: bool,
    pub event_type: AuthEventType,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthEventType {
    Login,
    Logout,
    FailedLogin,
    PrivilegeEscalation,
    SessionCreation,
    SessionTermination,
    Unknown,
}

/// Auth monitor
pub struct AuthMonitor {
    events: VecDeque<AuthEvent>,
    failed_attempts: VecDeque<AuthEvent>,
    suspicious_events: Vec<AuthEvent>,
    max_history: usize,
    max_failures: usize,
}

impl AuthMonitor {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(1000),
            failed_attempts: VecDeque::with_capacity(100),
            suspicious_events: Vec::new(),
            max_history: 1000,
            max_failures: 10,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Auth monitor active");
        Ok(())
    }

    /// Get authentication events
    pub async fn get_events(&mut self) -> Result<Vec<AuthEvent>> {
        let events = if cfg!(target_os = "windows") {
            self.get_windows_events().await?
        } else {
            self.get_unix_events().await?
        };

        // Store events
        for event in &events {
            self.events.push_back(event.clone());
            if self.events.len() > self.max_history {
                self.events.pop_front();
            }

            if !event.success {
                self.failed_attempts.push_back(event.clone());
                if self.failed_attempts.len() > self.max_failures {
                    self.failed_attempts.pop_front();
                }
            }
        }

        // Detect anomalies
        self.detect_anomalies(&events);

        Ok(events)
    }

    /// Get Unix auth events
    async fn get_unix_events(&self) -> Result<Vec<AuthEvent>> {
        let mut events = Vec::new();

        // Try /var/log/auth.log
        let output = Command::new("sh")
            .arg("-c")
            .arg("tail -n 50 /var/log/auth.log 2>/dev/null || tail -n 50 /var/log/secure 2>/dev/null")
            .output()
            .await?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            if let Some(event) = self.parse_auth_line(line) {
                events.push(event);
            }
        }

        Ok(events)
    }

    /// Get Windows auth events
    async fn get_windows_events(&self) -> Result<Vec<AuthEvent>> {
        let mut events = Vec::new();

        let output = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "Get-WinEvent -LogName Security -MaxEvents 50 | Select-Object -ExpandProperty Message",
            ])
            .output()
            .await?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            if let Some(event) = self.parse_auth_line(line) {
                events.push(event);
            }
        }

        Ok(events)
    }

    /// Parse a single auth line
    fn parse_auth_line(&self, line: &str) -> Option<AuthEvent> {
        // Parse timestamp
        let timestamp = self.parse_timestamp(line);
        
        // Parse event type
        let event_type = self.parse_event_type(line);
        
        // Parse username
        let username = self.extract_username(line);
        
        // Parse source IP
        let source_ip = self.extract_ip(line);

        // Determine success
        let success = !line.contains("FAILED") 
            && !line.contains("Authentication failure")
            && !line.contains("Invalid user");

        Some(AuthEvent {
            timestamp: timestamp.unwrap_or_else(Utc::now),
            username: username.unwrap_or_else(|| "unknown".to_string()),
            source_ip,
            success,
            event_type,
            details: line.to_string(),
        })
    }

    /// Parse timestamp from line
    fn parse_timestamp(&self, line: &str) -> Option<DateTime<Utc>> {
        // Try common timestamp formats
        let patterns = [
            r"\b\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2}\b",
            r"\b\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2}\b",
            r"\b\d{2}:\d{2}:\d{2}\b",
        ];

        for pattern in patterns {
            if let Some(cap) = regex::Regex::new(pattern).unwrap().find(line) {
                let ts_str = cap.as_str();
                if let Ok(ts) = chrono::NaiveDateTime::parse_from_str(ts_str, "%b %d %H:%M:%S") {
                    return Some(ts.and_local_timezone(Local).unwrap().with_timezone(&Utc));
                }
            }
        }

        None
    }

    /// Parse event type
    fn parse_event_type(&self, line: &str) -> AuthEventType {
        if line.contains("session opened") || line.contains("Accepted") {
            AuthEventType::Login
        } else if line.contains("session closed") || line.contains("logout") {
            AuthEventType::Logout
        } else if line.contains("FAILED") || line.contains("Invalid user") {
            AuthEventType::FailedLogin
        } else if line.contains("sudo") || line.contains("su") {
            AuthEventType::PrivilegeEscalation
        } else {
            AuthEventType::Unknown
        }
    }

    /// Extract username from line
    fn extract_username(&self, line: &str) -> Option<String> {
        let patterns = [
            r"user[\s=]+([a-zA-Z0-9_-]+)",
            r"for\s+([a-zA-Z0-9_-]+)",
            r"by\s+([a-zA-Z0-9_-]+)",
        ];

        for pattern in patterns {
            if let Some(cap) = regex::Regex::new(pattern).unwrap().captures(line) {
                if let Some(username) = cap.get(1) {
                    return Some(username.as_str().to_string());
                }
            }
        }

        // Check for ssh format
        if let Some(username) = regex::Regex::new(r"sshd\[.*?\]:.*?for\s+([a-zA-Z0-9_-]+)").unwrap()
            .captures(line)
            .and_then(|cap| cap.get(1))
        {
            return Some(username.as_str().to_string());
        }

        None
    }

    /// Extract IP address from line
    fn extract_ip(&self, line: &str) -> Option<String> {
        let re = regex::Regex::new(r"\b(\d{1,3}\.){3}\d{1,3}\b").unwrap();
        re.captures(line)
            .and_then(|cap| cap.get(0))
            .map(|ip| ip.as_str().to_string())
    }

    /// Detect anomalies in auth events
    fn detect_anomalies(&mut self, events: &[AuthEvent]) {
        // Brute force detection
        let failed_count = events.iter()
            .filter(|e| !e.success)
            .count();

        if failed_count > 5 {
            tracing::warn!("Possible brute force: {} failed attempts", failed_count);
            self.suspicious_events.extend(events.iter()
                .filter(|e| !e.success)
                .cloned()
                .collect::<Vec<_>>());
        }

        // Multiple sources detection
        let mut ips = std::collections::HashSet::new();
        for event in events {
            if let Some(ip) = &event.source_ip {
                ips.insert(ip.clone());
            }
        }
        if ips.len() > 3 {
            tracing::warn!("Multiple source IPs detected: {} unique IPs", ips.len());
        }

        // Privilege escalation detection
        for event in events {
            if event.event_type == AuthEventType::PrivilegeEscalation {
                tracing::warn!("Privilege escalation detected: {}", event.username);
                self.suspicious_events.push(event.clone());
            }
        }
    }

    /// Get suspicious events
    pub fn get_suspicious(&self) -> &[AuthEvent] {
        &self.suspicious_events
    }

    /// Clear suspicious list
    pub fn clear_suspicious(&mut self) {
        self.suspicious_events.clear();
    }
}