use crate::perception::PerceptionEngine;
use crate::actions::ActionEngine;
use crate::stealth::StealthEngine;

pub mod senses;
pub mod brain;
pub mod reflexes;

pub struct GhostAgent {
    perception: PerceptionEngine,
    actions: ActionEngine,
    stealth: StealthEngine,
    running: bool,
    stealth_enabled: bool,
}

impl GhostAgent {
    pub fn new() -> Self {
        Self {
            perception: PerceptionEngine::new(),
            actions: ActionEngine::new(),
            stealth: StealthEngine::new(),
            running: false,
            stealth_enabled: false,
        }
    }

    pub fn enable_stealth(&mut self) {
        self.stealth_enabled = true;
    }

    pub async fn deploy(&mut self) -> Result<(), anyhow::Error> {
        if self.stealth_enabled {
            self.stealth.hide_process()?;
            self.stealth.hide_memory()?;
            self.stealth.obfuscate_traffic()?;
        }
        
        self.perception.start_monitoring().await?;
        self.running = true;
        
        tracing::info!("👻 Ghost deployed successfully");
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        tracing::info!("Ghost is watching...");
        
        while self.running {
            // Scan for threats
            let threats = self.perception.scan().await?;
            
            // Process each threat
            for threat in threats {
                let action = self.actions.evaluate(&threat).await;
                let result = self.actions.execute(action).await?;
                
                if self.stealth_enabled {
                    // Clean up traces
                    self.stealth.clean_logs()?;
                }
                
                tracing::debug!("Action result: {:?}", result);
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
        
        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.running = false;
        tracing::info!("Ghost shutting down");
    }
}