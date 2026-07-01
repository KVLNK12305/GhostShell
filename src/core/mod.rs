//! Core Utilities - Ghost's Foundation
//!
//! This module provides the core infrastructure:
//! - Configuration management
//! - Cryptography
//! - Logging
//! - Error handling
//! - System utilities

pub mod config;
pub mod crypto;
pub mod logger;

use anyhow::Result;

/// Core engine that ties all utilities together
pub struct CoreEngine {
    pub config: config::Config,
    pub crypto: crypto::CryptoEngine,
}

impl CoreEngine {
    /// Initialize the core engine
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: config::Config::load()?,
            crypto: crypto::CryptoEngine::new(),
        })
    }

    /// Get the configuration
    pub fn config(&self) -> &config::Config {
        &self.config
    }

    /// Get the crypto engine
    pub fn crypto(&self) -> &crypto::CryptoEngine {
        &self.crypto
    }

    /// Reload configuration
    pub fn reload_config(&mut self) -> Result<()> {
        self.config = config::Config::load()?;
        Ok(())
    }
}

/// Core error types
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_yaml::Error),
}

/// Core result type
pub type CoreResult<T> = Result<T, CoreError>;