//! Team Exporter - Ghost Syncs with the Team
//!
//! Provides:
//! - Team synchronization
//! - Collective intelligence
//! - Shared threat database
//! - Peer-to-peer communication

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::perception::Threat;

/// Team member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub id: String,
    pub name: String,
    pub role: String,
    pub status: String,
    pub last_seen: DateTime<Utc>,
}

/// Team sync message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamSyncMessage {
    pub id: String,
    pub sender: String,
    pub timestamp: DateTime<Utc>,
    pub message_type: SyncType,
    pub payload: String,
}

/// Sync types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncType {
    Threat,
    Status,
    Intel,
    Alert,
    Heartbeat,
}

/// Team exporter
pub struct TeamExporter {
    team_id: String,
    members: Vec<TeamMember>,
    messages: Vec<TeamSyncMessage>,
    max_messages: usize,
    sync_enabled: bool,
}

impl TeamExporter {
    /// Create a new team exporter
    pub fn new() -> Self {
        Self {
            team_id: uuid::Uuid::new_v4().to_string(),
            members: Vec::new(),
            messages: Vec::new(),
            max_messages: 1000,
            sync_enabled: false,
        }
    }

    /// Enable team synchronization
    pub fn enable_sync(&mut self) {
        self.sync_enabled = true;
        tracing::info!("Team sync enabled");
    }

    /// Disable team synchronization
    pub fn disable_sync(&mut self) {
        self.sync_enabled = false;
        tracing::info!("Team sync disabled");
    }

    /// Sync a threat with the team
    pub async fn sync_threat(&self, threat: &Threat) -> Result<()> {
        if !self.sync_enabled {
            return Ok(());
        }

        tracing::info!("Syncing threat with team: {}", threat.id);

        let message = TeamSyncMessage {
            id: uuid::Uuid::new_v4().to_string(),
            sender: self.team_id.clone(),
            timestamp: Utc::now(),
            message_type: SyncType::Threat,
            payload: serde_json::to_string(threat)?,
        };

        self.send_message(message).await?;
        Ok(())
    }

    /// Sync status with the team
    pub async fn sync_status(&self, status: &str) -> Result<()> {
        if !self.sync_enabled {
            return Ok(());
        }

        tracing::info!("Syncing status with team");

        let message = TeamSyncMessage {
            id: uuid::Uuid::new_v4().to_string(),
            sender: self.team_id.clone(),
            timestamp: Utc::now(),
            message_type: SyncType::Status,
            payload: status.to_string(),
        };

        self.send_message(message).await?;
        Ok(())
    }

    /// Send a sync message
    async fn send_message(&self, message: TeamSyncMessage) -> Result<()> {
        // In a real implementation, this would send to a team server
        // For now, just log and store locally
        tracing::debug!("Team sync message: {:?}", message);

        // Store locally (would normally send to team server)
        self.store_message(message).await?;

        Ok(())
    }

    /// Store a message locally
    async fn store_message(&self, message: TeamSyncMessage) -> Result<()> {
        // Create messages directory
        tokio::fs::create_dir_all("messages/").await?;

        // Save message
        let filename = format!(
            "messages/{}_{}.json",
            message.timestamp.format("%Y%m%d_%H%M%S"),
            message.id.chars().take(8).collect::<String>()
        );

        let json = serde_json::to_string_pretty(&message)?;
        tokio::fs::write(&filename, json).await?;

        Ok(())
    }

    /// Add a team member
    pub fn add_member(&mut self, member: TeamMember) {
        self.members.push(member);
        tracing::debug!("Team member added");
    }

    /// Remove a team member
    pub fn remove_member(&mut self, id: &str) {
        self.members.retain(|m| m.id != id);
        tracing::debug!("Team member removed: {}", id);
    }

    /// Get all members
    pub fn get_members(&self) -> &[TeamMember] {
        &self.members
    }

    /// Get all messages
    pub fn get_messages(&self) -> &[TeamSyncMessage] {
        &self.messages
    }

    /// Clear old messages
    pub fn clear_old_messages(&mut self) {
        if self.messages.len() > self.max_messages {
            self.messages.drain(0..(self.messages.len() - self.max_messages));
        }
    }

    /// Set team ID
    pub fn set_team_id(&mut self, id: impl Into<String>) {
        self.team_id = id.into();
    }

    /// Get team ID
    pub fn get_team_id(&self) -> &str {
        &self.team_id
    }

    /// Check if sync is enabled
    pub fn is_sync_enabled(&self) -> bool {
        self.sync_enabled
    }

    /// Generate team heartbeat
    pub async fn heartbeat(&self) -> Result<()> {
        if !self.sync_enabled {
            return Ok(());
        }

        let message = TeamSyncMessage {
            id: uuid::Uuid::new_v4().to_string(),
            sender: self.team_id.clone(),
            timestamp: Utc::now(),
            message_type: SyncType::Heartbeat,
            payload: "alive".to_string(),
        };

        self.send_message(message).await?;
        Ok(())
    }
}

impl Default for TeamExporter {
    fn default() -> Self {
        Self::new()
    }
}