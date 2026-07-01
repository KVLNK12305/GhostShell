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
        let actions = self.actions.evaluate(std::slice::from_ref(threat)).await?;
        self.actions.execute_all(actions).await?;
        Ok(())
    }
}