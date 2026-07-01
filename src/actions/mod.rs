//! Action Engine - Ghost's Reflexes
//! 
//! This module contains all actions Ghost can take:
//! - Probing: Active reconnaissance
//! - Countering: Defensive/offensive operations
//! - Deceiving: Deception campaigns
//! - Neutralizing: Threat elimination

mod probe;
mod counter;
mod deceive;
mod neutralize;

use anyhow::Result;
use crate::perception::Threat;
use crate::perception::ThreatSeverity;
use crate::actions::probe::ProbeEngine;
use crate::actions::counter::CounterEngine;
use crate::actions::deceive::DeceiveEngine;
use crate::actions::neutralize::NeutralizeEngine;

pub use probe::ProbeEngine;
pub use counter::CounterEngine;
pub use deceive::DeceiveEngine;
pub use neutralize::NeutralizeEngine;

/// All possible actions Ghost can take
#[derive(Debug, Clone)]
pub enum Action {
    /// Probe the threat for more intelligence
    Probe(Threat),
    /// Counter the threat's operations
    Counter(Threat),
    /// Deceive the threat actor
    Deceive(Threat),
    /// Neutralize the threat completely
    Neutralize(Threat),
    /// Observe and gather intelligence
    Observe(Threat),
    /// Report intelligence to command
    Report(Threat),
}

/// Action result with metadata
#[derive(Debug, Clone)]
pub struct ActionResult {
    pub action: Action,
    pub success: bool,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub evidence: Vec<String>,
}

/// Main Action Engine orchestrator
pub struct ActionEngine {
    probe: ProbeEngine,
    counter: CounterEngine,
    deceive: DeceiveEngine,
    neutralize: NeutralizeEngine,
    action_history: Vec<ActionResult>,
    max_history: usize,
}

impl ActionEngine {
    pub fn new() -> Self {
        Self {
            probe: ProbeEngine::new(),
            counter: CounterEngine::new(),
            deceive: DeceiveEngine::new(),
            neutralize: NeutralizeEngine::new(),
            action_history: Vec::new(),
            max_history: 1000,
        }
    }

    /// Evaluate threats and decide actions
    pub async fn evaluate(&mut self, threats: &[Threat]) -> Result<Vec<Action>> {
        let mut actions = Vec::new();

        for threat in threats {
            let action = match threat.severity {
                ThreatSeverity::Critical => {
                    // Ghost neutralizes critical threats immediately
                    Action::Neutralize(threat.clone())
                }
                ThreatSeverity::High => {
                    // Ghost counters high threats
                    Action::Counter(threat.clone())
                }
                ThreatSeverity::Medium => {
                    // Ghost probes medium threats
                    Action::Probe(threat.clone())
                }
                ThreatSeverity::Low => {
                    // Ghost observes low threats
                    Action::Observe(threat.clone())
                }
                ThreatSeverity::Info => {
                    // Ghost reports informational threats
                    Action::Report(threat.clone())
                }
            };
            actions.push(action);
        }

        // Prioritize actions based on threat severity
        self.prioritize_actions(&mut actions).await;
        
        Ok(actions)
    }

    /// Execute a single action
    pub async fn execute(&mut self, action: Action) -> Result<ActionResult> {
        let result = match &action {
            Action::Probe(threat) => {
                self.probe.execute(threat).await?
            }
            Action::Counter(threat) => {
                self.counter.execute(threat).await?
            }
            Action::Deceive(threat) => {
                self.deceive.execute(threat).await?
            }
            Action::Neutralize(threat) => {
                self.neutralize.execute(threat).await?
            }
            Action::Observe(threat) => {
                // Ghost just watches and logs
                tracing::debug!("Observing threat: {}", threat.description);
                ActionResult {
                    action,
                    success: true,
                    message: format!("Observed threat: {}", threat.description),
                    timestamp: chrono::Utc::now(),
                    evidence: vec![threat.description.clone()],
                }
            }
            Action::Report(threat) => {
                // Ghost reports intelligence
                ActionResult {
                    action,
                    success: true,
                    message: format!("Reported threat: {}", threat.id),
                    timestamp: chrono::Utc::now(),
                    evidence: vec![format!("Threat ID: {}", threat.id)],
                }
            }
        };

        // Store history
        self.action_history.push(result.clone());
        if self.action_history.len() > self.max_history {
            self.action_history.remove(0);
        }

        Ok(result)
    }

    /// Execute multiple actions in sequence
    pub async fn execute_all(&mut self, actions: Vec<Action>) -> Result<Vec<ActionResult>> {
        let mut results = Vec::new();
        for action in actions {
            let result = self.execute(action).await?;
            results.push(result);
        }
        Ok(results)
    }

    /// Prioritize actions by threat severity
    async fn prioritize_actions(&self, actions: &mut Vec<Action>) {
        // Critical actions first
        actions.sort_by(|a, b| {
            let severity_a = match a {
                Action::Neutralize(_) => 5,
                Action::Counter(_) => 4,
                Action::Deceive(_) => 3,
                Action::Probe(_) => 2,
                Action::Report(_) => 1,
                Action::Observe(_) => 0,
            };
            let severity_b = match b {
                Action::Neutralize(_) => 5,
                Action::Counter(_) => 4,
                Action::Deceive(_) => 3,
                Action::Probe(_) => 2,
                Action::Report(_) => 1,
                Action::Observe(_) => 0,
            };
            severity_b.cmp(&severity_a)
        });
    }

    /// Get action history
    pub fn get_history(&self) -> &[ActionResult] {
        &self.action_history
    }

    /// Clear action history
    pub fn clear_history(&mut self) {
        self.action_history.clear();
    }
}