//! Exporters - Ghost Reports Intelligence
//!
//! Provides:
//! - Report generation
//! - Team synchronization
//! - Covert communication
//! - Intelligence sharing

pub mod report;
pub mod team;

use anyhow::Result;
use crate::perception::Threat;

/// Main exporter engine
pub struct ExporterEngine {
    pub report: report::ReportExporter,
    pub team: team::TeamExporter,
}

impl ExporterEngine {
    /// Create a new exporter engine
    pub fn new() -> Self {
        Self {
            report: report::ReportExporter::new(),
            team: team::TeamExporter::new(),
        }
    }

    /// Export threat intelligence
    pub async fn export_threat(&self, threat: &Threat) -> Result<()> {
        self.report.export_threat(threat).await?;
        self.team.sync_threat(threat).await?;
        Ok(())
    }

    /// Export multiple threats
    pub async fn export_threats(&self, threats: &[Threat]) -> Result<()> {
        for threat in threats {
            self.export_threat(threat).await?;
        }
        Ok(())
    }

    /// Export system status
    pub async fn export_status(&self, status: &str) -> Result<()> {
        self.report.export_status(status).await?;
        Ok(())
    }
}