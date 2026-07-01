//! Report Exporter - Ghost Generates Intelligence Reports
//!
//! Provides:
//! - Threat reports
//! - Status reports
//! - JSON/YAML export
//! - Covert reporting

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::perception::Threat;

/// Report types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportType {
    Threat,
    Status,
    Intelligence,
    Summary,
}

/// Intelligence report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntelligenceReport {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub report_type: ReportType,
    pub threats: Vec<Threat>,
    pub summary: String,
    pub metadata: HashMap<String, String>,
}

/// Report exporter
pub struct ReportExporter {
    reports: Vec<IntelligenceReport>,
    max_reports: usize,
    export_path: String,
}

impl ReportExporter {
    /// Create a new report exporter
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
            max_reports: 100,
            export_path: "reports/".to_string(),
        }
    }

    /// Export a threat report
    pub async fn export_threat(&self, threat: &Threat) -> Result<()> {
        tracing::info!("Exporting threat report: {}", threat.id);
        
        let report = IntelligenceReport {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            report_type: ReportType::Threat,
            threats: vec![threat.clone()],
            summary: format!("Threat detected: {}", threat.description),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("source".to_string(), threat.source.clone());
                meta.insert("severity".to_string(), format!("{:?}", threat.severity));
                meta
            },
        };

        self.save_report(&report).await?;
        Ok(())
    }

    /// Export status report
    pub async fn export_status(&self, status: &str) -> Result<()> {
        tracing::info!("Exporting status report");
        
        let report = IntelligenceReport {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            report_type: ReportType::Status,
            threats: Vec::new(),
            summary: status.to_string(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "status".to_string());
                meta
            },
        };

        self.save_report(&report).await?;
        Ok(())
    }

    /// Save a report
    async fn save_report(&self, report: &IntelligenceReport) -> Result<()> {
        // Create reports directory if it doesn't exist
        tokio::fs::create_dir_all(&self.export_path).await?;

        // Generate filename
        let filename = format!(
            "{}/{}_{}.json",
            self.export_path,
            report.timestamp.format("%Y%m%d_%H%M%S"),
            report.id.chars().take(8).collect::<String>()
        );

        // Serialize and save
        let json = serde_json::to_string_pretty(report)?;
        tokio::fs::write(&filename, json).await?;

        tracing::debug!("Report saved to: {}", filename);
        Ok(())
    }

    /// Export as JSON
    pub fn to_json(&self, report: &IntelligenceReport) -> Result<String> {
        Ok(serde_json::to_string_pretty(report)?)
    }

    /// Export as YAML
    pub fn to_yaml(&self, report: &IntelligenceReport) -> Result<String> {
        Ok(serde_yaml::to_string(report)?)
    }

    /// Export as plain text
    pub fn to_text(&self, report: &IntelligenceReport) -> String {
        let mut output = String::new();
        output.push_str(&format!("=== GHOST INTELLIGENCE REPORT ===\n"));
        output.push_str(&format!("ID: {}\n", report.id));
        output.push_str(&format!("Time: {}\n", report.timestamp));
        output.push_str(&format!("Type: {:?}\n", report.report_type));
        output.push_str(&format!("Summary: {}\n", report.summary));
        output.push_str("\n--- Threats ---\n");
        for threat in &report.threats {
            output.push_str(&format!("  - {}: {}\n", threat.id, threat.description));
        }
        output
    }

    /// Get all reports
    pub fn get_reports(&self) -> &[IntelligenceReport] {
        &self.reports
    }

    /// Clear old reports
    pub fn clear_old_reports(&mut self) {
        if self.reports.len() > self.max_reports {
            self.reports.drain(0..(self.reports.len() - self.max_reports));
        }
    }

    /// Set export path
    pub fn set_export_path(&mut self, path: impl Into<String>) {
        self.export_path = path.into();
    }

    /// Set max reports
    pub fn set_max_reports(&mut self, max: usize) {
        self.max_reports = max;
    }
}

impl Default for ReportExporter {
    fn default() -> Self {
        Self::new()
    }
}