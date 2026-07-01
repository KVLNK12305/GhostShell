//! System Perception - Ghost sees system activity
//!
//! Monitors:
//! - Running processes
//! - System resources
//! - File system changes
//! - User activity

use anyhow::Result;
use std::collections::HashMap;
use sysinfo::{System, ProcessExt, SystemExt};

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub parent_pid: Option<u32>,
    pub parent_name: Option<String>,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub command_line: String,
    pub executable_path: String,
    pub user: String,
}

/// System monitor
pub struct SystemMonitor {
    system: System,
    process_history: HashMap<u32, ProcessInfo>,
    suspicious_processes: Vec<ProcessInfo>,
    max_history: usize,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            process_history: HashMap::new(),
            suspicious_processes: Vec::new(),
            max_history: 1000,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("System monitor active");
        Ok(())
    }

    /// Get current process list
    pub async fn get_processes(&mut self) -> Result<Vec<ProcessInfo>> {
        // Refresh system info
        self.system.refresh_all();
        
        let mut processes = Vec::new();

        for (pid, process) in self.system.processes() {
            let pid_u32 = usize::from(*pid) as u32;
            let info = ProcessInfo {
                pid: pid_u32,
                name: process.name().to_string(),
                parent_pid: process.parent().map(|p| usize::from(p) as u32),
                parent_name: process.parent().and_then(|p| {
                    self.system.process(p).map(|pp| pp.name().to_string())
                }),
                cpu_usage: process.cpu_usage(),
                memory_usage: process.memory(),
                command_line: process.cmd().join(" "),
                executable_path: process.exe().to_string_lossy().to_string(),
                user: self.get_process_user(pid_u32),
            };
            
            processes.push(info.clone());
            
            // Store in history
            self.process_history.insert(pid_u32, info);
            
            // Trim history
            if self.process_history.len() > self.max_history {
                // Remove oldest entries
                let oldest = self.process_history.keys().cloned().collect::<Vec<_>>();
                if let Some(&first) = oldest.first() {
                    self.process_history.remove(&first);
                }
            }
        }

        // Detect suspicious processes
        self.detect_suspicious(&processes);

        Ok(processes)
    }

    /// Get process user
    fn get_process_user(&self, pid: u32) -> String {
        #[cfg(unix)]
        {
            use std::fs;
            if let Ok(contents) = fs::read_to_string(format!("/proc/{}/status", pid)) {
                for line in contents.lines() {
                    if line.starts_with("Uid:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(uid) = parts[1].parse::<u32>() {
                                return self.get_username(uid);
                            }
                        }
                    }
                }
            }
        }
        "unknown".to_string()
    }

    /// Get username from UID
    fn get_username(&self, uid: u32) -> String {
        #[cfg(unix)]
        {
            use std::fs;
            if let Ok(entries) = fs::read_to_string("/etc/passwd") {
                for line in entries.lines() {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 3 {
                        if let Ok(u) = parts[2].parse::<u32>() {
                            if u == uid {
                                return parts[0].to_string();
                            }
                        }
                    }
                }
            }
        }
        format!("uid:{}", uid)
    }

    /// Detect suspicious processes
    fn detect_suspicious(&mut self, processes: &[ProcessInfo]) {
        let suspicious_names = vec![
            "nc", "netcat", "ncat", "telnet", "ssh",
            "crypt", "miner", "xmrig", "cpuminer",
            "meterpreter", "shell", "cmd", "powershell",
        ];

        for process in processes {
            let name_lower = process.name.to_lowercase();
            
            // Check suspicious names
            for suspicious in &suspicious_names {
                if name_lower.contains(suspicious) {
                    tracing::warn!("Suspicious process: {} (PID: {})", process.name, process.pid);
                    self.suspicious_processes.push(process.clone());
                }
            }

            // Check hidden processes
            if process.name.contains(" ") && process.name.len() < 10 {
                tracing::warn!("Potential hidden process: {}", process.name);
                self.suspicious_processes.push(process.clone());
            }

            // Check high resource usage
            if process.cpu_usage > 80.0 {
                tracing::warn!("High CPU usage: {} ({:.1}%)", process.name, process.cpu_usage);
            }
        }
    }

    /// Get suspicious processes
    pub fn get_suspicious(&self) -> &[ProcessInfo] {
        &self.suspicious_processes
    }

    /// Clear suspicious list
    pub fn clear_suspicious(&mut self) {
        self.suspicious_processes.clear();
    }
}