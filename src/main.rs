use ghostshell::*;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "ghost", about = "Silent Cyber Operations Agent")]
struct Cli {
    #[arg(short, long)]
    deploy: bool,

    #[arg(short, long)]
    stealth: bool,

    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    core::logger::init();
    
    if cli.deploy {
        let mut agent = agent::GhostAgent::new();
        
        if cli.stealth {
            agent.enable_stealth();
        }
        
        agent.deploy().await?;
        agent.run().await?;
    } else {
        println!("👻 Project Ghost");
        println!("Usage: ghost --deploy [--stealth]");
        println!("Run with --help for more options");
    }
    
    Ok(())
}