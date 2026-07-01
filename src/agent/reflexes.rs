//! Ghost's Reflexes - Quick responses

use crate::actions::ActionEngine;

pub struct Reflexes {
    actions: ActionEngine,
}

impl Reflexes {
    pub fn new() -> Self {
        Self {
            actions: ActionEngine::new(),
        }
    }

    pub async fn react(&mut self, threat: &crate::perception::Threat) -> Result<(), anyhow::Error> {
        let action = self.actions.evaluate(threat).await;
        self.actions.execute(action).await?;
        Ok(())
    }
}