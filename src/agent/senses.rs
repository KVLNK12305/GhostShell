//! Ghost's Senses - Perception capabilities

use crate::perception::PerceptionEngine;

pub struct Senses {
    perception: PerceptionEngine,
}

impl Senses {
    pub fn new() -> Self {
        Self {
            perception: PerceptionEngine::new(),
        }
    }

    pub async fn perceive(&mut self) -> Vec<crate::perception::Threat> {
        self.perception.scan().await.unwrap_or_default()
    }
}