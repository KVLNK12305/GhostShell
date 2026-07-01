//! Configuration Management - Ghost's Settings
//!
//! Handles:
//! - Loading configuration from files
//! - Environment variable overrides
//! - Default configuration
//! - Configuration validation

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Agent configuration
    pub agent: AgentConfig,
    
    /// Perception engine configuration
    pub perception: PerceptionConfig,
    
    /// Action engine configuration
    pub actions: ActionsConfig,
    
    /// Stealth engine configuration
    pub stealth: StealthConfig,
    
    /// Reporting configuration
    pub reporting: ReportingConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent name
    pub name: String,
    
    /// Agent version
    pub version: String,
    
    /// Enable autonomous mode
    pub autonomous: bool,
    
    /// Scan interval in seconds
    pub scan_interval: u64,
    
    /// Self-destruct on detection
    pub self_destruct_on_detect: bool,
    
    /// Maximum concurrent operations
    pub max_concurrent_ops: usize,
}

/// Perception configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionConfig {
    /// Enable network monitoring
    pub network: bool,
    
    /// Enable system monitoring
    pub system: bool,
    
    /// Enable auth monitoring
    pub auth: bool,
    
    /// Enable filesystem monitoring
    pub filesystem: bool,
    
    /// Scan interval in seconds
    pub scan_interval: u64,
    
    /// Anomaly detection threshold (0.0 - 1.0)
    pub anomaly_threshold: f64,
    
    /// Maximum history size
    pub max_history: usize,
}

/// Actions configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsConfig {
    /// Auto-neutralize threats
    pub auto_neutralize: bool,
    
    /// Probe depth (light, medium, aggressive)
    pub probe_depth: String,
    
    /// Counter strategy (passive, active, adaptive)
    pub counter_strategy: String,
    
    /// Enable deception
    pub deception_enabled: bool,
    
    /// Maximum actions per scan
    pub max_actions_per_scan: usize,
}

/// Stealth configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    /// Hide process from system monitors
    pub hide_process: bool,
    
    /// Hide memory regions
    pub hide_memory: bool,
    
    /// Obfuscate network traffic
    pub obfuscate_traffic: bool,
    
    /// Clean logs after operations
    pub clean_logs: bool,
    
    /// Self-destruct when detected
    pub self_destruct_on_detect: bool,
    
    /// Encryption level (light, standard, military)
    pub encryption_level: String,
}

/// Reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    /// Report method (local, remote, covert)
    pub method: String,
    
    /// Encryption algorithm
    pub encryption: String,
    
    /// Report destination
    pub destination: String,
    
    /// Report frequency in seconds
    pub frequency: u64,
    
    /// Enable compression
    pub compression: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Encryption key rotation interval in seconds
    pub encryption_key_rotation: u64,
    
    /// Maximum authentication failures
    pub max_failures: u64,
    
    /// Enable integrity checking
    pub integrity_check: bool,
}

impl Config {
    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let config_paths = vec![
            "config/ghost.yaml",
            "ghost.yaml",
            "/etc/ghost/config.yaml",
            "~/.config/ghost/config.yaml",
        ];

        for path in config_paths {
            let expanded_path = shellexpand::tilde(path).to_string();
            let path = PathBuf::from(expanded_path);
            
            if path.exists() {
                let contents = std::fs::read_to_string(&path)?;
                let config: Config = serde_yaml::from_str(&contents)?;
                return Ok(config);
            }
        }

        // Return default config
        Ok(Config::default())
    }

    /// Save configuration to file
    pub fn save(&self, path: Option<&str>) -> Result<()> {
        let save_path = path.unwrap_or("config/ghost.yaml");
        let contents = serde_yaml::to_string(self)?;
        std::fs::write(save_path, contents)?;
        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate agent config
        if self.agent.scan_interval == 0 {
            return Err(anyhow::anyhow!("scan_interval must be greater than 0"));
        }

        // Validate perception config
        if self.perception.anomaly_threshold < 0.0 || self.perception.anomaly_threshold > 1.0 {
            return Err(anyhow::anyhow!("anomaly_threshold must be between 0.0 and 1.0"));
        }

        // Validate stealth config
        if !["light", "standard", "military"].contains(&self.stealth.encryption_level.as_str()) {
            return Err(anyhow::anyhow!("invalid encryption_level"));
        }

        Ok(())
    }

    /// Get config directory
    pub fn get_config_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".config").join("ghost")
    }

    /// Get data directory
    pub fn get_data_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".local").join("share").join("ghost")
    }

    /// Get log directory
    pub fn get_log_dir() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(".local").join("log").join("ghost")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent: AgentConfig {
                name: "ghost-agent".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                autonomous: true,
                scan_interval: 5,
                self_destruct_on_detect: true,
                max_concurrent_ops: 10,
            },
            perception: PerceptionConfig {
                network: true,
                system: true,
                auth: true,
                filesystem: true,
                scan_interval: 5,
                anomaly_threshold: 0.7,
                max_history: 10000,
            },
            actions: ActionsConfig {
                auto_neutralize: true,
                probe_depth: "aggressive".to_string(),
                counter_strategy: "adaptive".to_string(),
                deception_enabled: true,
                max_actions_per_scan: 10,
            },
            stealth: StealthConfig {
                hide_process: true,
                hide_memory: true,
                obfuscate_traffic: true,
                clean_logs: true,
                self_destruct_on_detect: true,
                encryption_level: "military".to_string(),
            },
            reporting: ReportingConfig {
                method: "covert".to_string(),
                encryption: "AES-256-GCM".to_string(),
                destination: "https://intel.ghost.local/report".to_string(),
                frequency: 60,
                compression: true,
            },
            security: SecurityConfig {
                encryption_key_rotation: 3600,
                max_failures: 5,
                integrity_check: true,
            },
        }
    }
}

/// Configuration builder for programmatic config creation
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    pub fn agent_name(mut self, name: impl Into<String>) -> Self {
        self.config.agent.name = name.into();
        self
    }

    pub fn autonomous(mut self, autonomous: bool) -> Self {
        self.config.agent.autonomous = autonomous;
        self
    }

    pub fn scan_interval(mut self, interval: u64) -> Self {
        self.config.agent.scan_interval = interval;
        self
    }

    pub fn stealth_level(mut self, level: impl Into<String>) -> Self {
        self.config.stealth.encryption_level = level.into();
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}