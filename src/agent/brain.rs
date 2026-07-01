//! Ghost's Brain - Decision making

use crate::perception::Threat;
use crate::actions::Action;

pub struct Brain {
    experience: Vec<Threat>,
    max_memory: usize,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            experience: Vec::new(),
            max_memory: 1000,
        }
    }

    pub fn decide(&mut self, threat: &Threat) -> Action {
        // Learn from experience
        self.experience.push(threat.clone());
        if self.experience.len() > self.max_memory {
            self.experience.remove(0);
        }

        // Simple decision logic
        match threat.severity {
            crate::perception::ThreatSeverity::Critical => Action::Neutralize(threat.clone()),
            crate::perception::ThreatSeverity::High => Action::Counter(threat.clone()),
            crate::perception::ThreatSeverity::Medium => Action::Probe(threat.clone()),
            _ => Action::Observe(threat.clone()),
        }
    }
}