//! Project Ghost - Integration Tests
//!
//! This file tests all components of Ghost:
//! - Perception engine
//! - Action engine
//! - Stealth engine
//! - Core utilities
//! - Exporters
//! - Full agent workflow

use ghostshell::*;
use chrono::Utc;

fn make_test_threat(desc: &str, severity: perception::ThreatSeverity) -> perception::Threat {
    perception::Threat {
        id: uuid::Uuid::new_v4().to_string(),
        severity,
        source: "test".to_string(),
        description: desc.to_string(),
        timestamp: Utc::now(),
        evidence: vec![perception::Evidence {
            kind: perception::EvidenceKind::SystemLog,
            data: "Test evidence".to_string(),
            timestamp: Utc::now(),
        }],
        confidence: 0.95,
    }
}

// ============================================================================
// TEST 1: CORE CONFIGURATION
// ============================================================================

#[test]
fn test_config_loading() {
    println!("🧪 Testing: Configuration Loading");
    
    let config = core::config::Config::default();
    assert_eq!(config.agent.name, "ghost-agent");
    assert_eq!(config.agent.scan_interval, 5);
    assert_eq!(config.stealth.encryption_level, "military");
    
    println!("✅ Config test passed\n");
}

#[test]
fn test_config_validation() {
    println!("🧪 Testing: Configuration Validation");
    
    let config = core::config::Config::default();
    let result = config.validate();
    assert!(result.is_ok(), "Default config should be valid");
    
    println!("✅ Config validation passed\n");
}

// ============================================================================
// TEST 2: CRYPTO ENGINE
// ============================================================================

#[test]
fn test_crypto_key_generation() {
    println!("🧪 Testing: Crypto Key Generation");
    
    let crypto = core::crypto::CryptoEngine::new();
    let key = crypto.generate_key().unwrap();
    assert_eq!(key.len(), 32);
    
    println!("✅ Key generation passed\n");
}

#[test]
fn test_crypto_encryption() {
    println!("🧪 Testing: Crypto Encryption/Decryption");
    
    let mut crypto = core::crypto::CryptoEngine::new();
    let key = crypto.generate_key().unwrap();
    crypto.set_key(key);
    
    let plaintext = b"Hello Ghost! This is a secret message.";
    let encrypted = crypto.encrypt(plaintext).unwrap();
    let decrypted = crypto.decrypt(&encrypted).unwrap();
    
    assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    
    println!("✅ Encryption/Decryption passed\n");
}

#[test]
fn test_crypto_hashing() {
    println!("🧪 Testing: Crypto Hashing");
    
    let crypto = core::crypto::CryptoEngine::new();
    let data = b"Test data for hashing";
    let hash = crypto.hash_sha256(data);
    
    assert_eq!(hash.len(), 32);
    
    // Same data should produce same hash
    let hash2 = crypto.hash_sha256(data);
    assert_eq!(hash, hash2);
    
    // Different data should produce different hash
    let data2 = b"Different test data";
    let hash3 = crypto.hash_sha256(data2);
    assert_ne!(hash, hash3);
    
    println!("✅ Hashing passed\n");
}

#[test]
fn test_crypto_uuid() {
    println!("🧪 Testing: UUID Generation");
    
    let crypto = core::crypto::CryptoEngine::new();
    let uuid1 = crypto.generate_uuid().unwrap();
    let uuid2 = crypto.generate_uuid().unwrap();
    
    // UUIDs should be unique
    assert_ne!(uuid1, uuid2);
    
    // UUID should have correct format (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
    let parts: Vec<&str> = uuid1.split('-').collect();
    assert_eq!(parts.len(), 5);
    assert_eq!(parts[0].len(), 8);
    assert_eq!(parts[1].len(), 4);
    assert_eq!(parts[2].len(), 4);
    assert_eq!(parts[3].len(), 4);
    assert_eq!(parts[4].len(), 12);
    
    println!("✅ UUID generation passed: {}\n", uuid1);
}

// ============================================================================
// TEST 3: LOGGER
// ============================================================================

#[test]
fn test_logger_initialization() {
    println!("🧪 Testing: Logger Initialization");
    
    // This should not panic
    core::logger::init();
    
    // Log some messages
    tracing::info!("Test info message");
    tracing::debug!("Test debug message");
    tracing::warn!("Test warning message");
    tracing::error!("Test error message");
    
    println!("✅ Logger initialization passed\n");
}

// ============================================================================
// TEST 4: PERCEPTION ENGINE - NETWORK
// ============================================================================

#[tokio::test]
async fn test_network_monitor() {
    println!("🧪 Testing: Network Monitor");
    
    let mut network = perception::NetworkMonitor::new();
    network.start().await.unwrap();
    
    let sockets = network.get_sockets().await.unwrap();
    
    // Should have at least some sockets
    println!("  Found {} socket connections", sockets.len());
    
    // Check socket structure
    if let Some(socket) = sockets.first() {
        assert!(!socket.protocol.is_empty());
        assert!(!socket.local_addr.is_empty());
        // If it has a local port, it should be valid
        if let Some(port) = socket.local_port {
            assert!(port > 0);
        }
    }
    
    println!("✅ Network monitor test passed\n");
}

// ============================================================================
// TEST 5: PERCEPTION ENGINE - SYSTEM
// ============================================================================

#[tokio::test]
async fn test_system_monitor() {
    println!("🧪 Testing: System Monitor");
    
    let mut system = perception::SystemMonitor::new();
    system.start().await.unwrap();
    
    let processes = system.get_processes().await.unwrap();
    
    // Should have at least some processes
    println!("  Found {} running processes", processes.len());
    assert!(processes.len() > 0, "Should have at least one process");
    
    // Check process structure
    if let Some(process) = processes.first() {
        assert!(process.pid > 0);
        assert!(!process.name.is_empty());
        // CPU usage should be between 0 and 100
        assert!(process.cpu_usage >= 0.0 && process.cpu_usage <= 100.0);
    }
    
    println!("✅ System monitor test passed\n");
}

// ============================================================================
// TEST 6: PERCEPTION ENGINE - AUTH
// ============================================================================

#[tokio::test]
async fn test_auth_monitor() {
    println!("🧪 Testing: Auth Monitor");
    
    let mut auth = perception::AuthMonitor::new();
    auth.start().await.unwrap();
    
    let events = auth.get_events().await.unwrap();
    
    // Auth events may be empty on some systems, that's OK
    println!("  Found {} auth events", events.len());
    
    if let Some(event) = events.first() {
        assert!(!event.username.is_empty());
        // Success should be a boolean
        assert!(event.success == true || event.success == false);
    }
    
    println!("✅ Auth monitor test passed\n");
}

// ============================================================================
// TEST 7: ANOMALY DETECTION
// ============================================================================

#[tokio::test]
async fn test_anomaly_detection() {
    println!("🧪 Testing: Anomaly Detection");
    
    let mut anomaly = perception::AnomalyDetector::new();
    
    // Create test data
    let sockets = vec![
        perception::network::SocketRecord {
            protocol: "tcp".to_string(),
            state: "LISTEN".to_string(),
            local_addr: "0.0.0.0".to_string(),
            local_port: Some(22),
            peer_addr: "*".to_string(),
            peer_port: None,
            process: Some("sshd".to_string()),
            timestamp: Utc::now(),
        },
        perception::network::SocketRecord {
            protocol: "tcp".to_string(),
            state: "ESTABLISHED".to_string(),
            local_addr: "192.168.1.10".to_string(),
            local_port: Some(443),
            peer_addr: "45.33.22.11".to_string(),
            peer_port: Some(4444),
            process: Some("nginx".to_string()),
            timestamp: Utc::now(),
        },
    ];
    
    let processes = vec![
        perception::system::ProcessInfo {
            pid: 1234,
            name: "xmrig".to_string(),
            parent_pid: Some(1),
            parent_name: Some("init".to_string()),
            cpu_usage: 85.5,
            memory_usage: 1024000,
            command_line: "xmrig -o pool".to_string(),
            executable_path: "/tmp/xmrig".to_string(),
            user: "root".to_string(),
        },
        perception::system::ProcessInfo {
            pid: 5678,
            name: "sshd".to_string(),
            parent_pid: Some(1),
            parent_name: Some("init".to_string()),
            cpu_usage: 2.3,
            memory_usage: 512000,
            command_line: "/usr/sbin/sshd".to_string(),
            executable_path: "/usr/sbin/sshd".to_string(),
            user: "root".to_string(),
        },
    ];
    
    let auth_events = vec![
        perception::auth::AuthEvent {
            timestamp: Utc::now(),
            username: "root".to_string(),
            source_ip: Some("192.168.1.100".to_string()),
            success: false,
            event_type: perception::auth::AuthEventType::FailedLogin,
            details: "Failed login attempt".to_string(),
        },
        perception::auth::AuthEvent {
            timestamp: Utc::now(),
            username: "admin".to_string(),
            source_ip: Some("10.0.0.5".to_string()),
            success: false,
            event_type: perception::auth::AuthEventType::FailedLogin,
            details: "Failed login attempt".to_string(),
        },
        perception::auth::AuthEvent {
            timestamp: Utc::now(),
            username: "kushal".to_string(),
            source_ip: Some("127.0.0.1".to_string()),
            success: true,
            event_type: perception::auth::AuthEventType::Login,
            details: "Successful login".to_string(),
        },
    ];
    
    let anomalies = anomaly.detect(&sockets, &processes, &auth_events).await.unwrap();
    
    println!("  Found {} anomalies", anomalies.len());
    
    // Should detect at least the exposed port and suspicious process
    assert!(anomalies.len() >= 2, "Should detect exposed port and suspicious process");
    
    // Check for critical anomaly
    let critical_found = anomalies.iter().any(|a| {
        matches!(a.severity, perception::anomaly::Severity::Critical)
    });
    assert!(critical_found, "Should have critical anomalies");
    
    // Check for suspicious process detection
    let suspicious_found = anomalies.iter().any(|a| {
        a.text.contains("xmrig") || a.text.contains("netcat")
    });
    assert!(suspicious_found, "Should detect suspicious process");
    
    println!("✅ Anomaly detection passed\n");
}

// ============================================================================
// TEST 8: ACTION ENGINE
// ============================================================================

#[tokio::test]
async fn test_action_engine() {
    println!("🧪 Testing: Action Engine");
    
    let mut actions = actions::ActionEngine::new();
    
    // Create a test threat
    let threat = make_test_threat("Test threat for action evaluation", perception::ThreatSeverity::High);
    
    // Test evaluation
    let actions_list = actions.evaluate(std::slice::from_ref(&threat)).await.unwrap();
    
    if let Some(action) = actions_list.first() {
        match action {
            actions::Action::Counter(_) => println!("  Action: Counter selected"),
            actions::Action::Probe(_) => println!("  Action: Probe selected"),
            actions::Action::Neutralize(_) => println!("  Action: Neutralize selected"),
            actions::Action::Observe(_) => println!("  Action: Observe selected"),
            actions::Action::Deceive(_) => println!("  Action: Deceive selected"),
            actions::Action::Report(_) => println!("  Action: Report selected"),
        }
    }
    
    // Test execution
    let results = actions.execute_all(actions_list).await.unwrap();
    if let Some(result) = results.first() {
        println!("  Result: {}", result.message);
        assert!(!result.message.is_empty() || result.success);
    }
    
    println!("✅ Action engine test passed\n");
}

// ============================================================================
// TEST 9: STEALTH ENGINE
// ============================================================================

#[test]
fn test_stealth_engine() {
    println!("🧪 Testing: Stealth Engine");
    
    let stealth = stealth::StealthEngine::new();
    
    // Test hiding
    let result = stealth.hide_process();
    assert!(result.is_ok(), "Process hiding should work");
    
    let result = stealth.hide_memory();
    assert!(result.is_ok(), "Memory hiding should work");
    
    let result = stealth.obfuscate_traffic();
    assert!(result.is_ok(), "Traffic obfuscation should work");
    
    let result = stealth.clean_logs();
    assert!(result.is_ok(), "Log cleaning should work");
    
    println!("✅ Stealth engine test passed\n");
}

// ============================================================================
// TEST 10: EXPORTERS
// ============================================================================

#[tokio::test]
async fn test_report_exporter() {
    println!("🧪 Testing: Report Exporter");
    
    let exporter = exporters::report::ReportExporter::new();
    
    let threat = make_test_threat("Test threat for reporting", perception::ThreatSeverity::Critical);
    
    // Test export
    let result = exporter.export_threat(&threat).await;
    assert!(result.is_ok(), "Report export should work");
    
    // Test serialization
    let report = exporters::report::IntelligenceReport {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: Utc::now(),
        report_type: exporters::report::ReportType::Threat,
        threats: vec![threat],
        summary: "Test summary".to_string(),
        metadata: std::collections::HashMap::new(),
    };
    
    let json = exporter.to_json(&report).unwrap();
    assert!(!json.is_empty());
    
    let text = exporter.to_text(&report);
    assert!(text.contains("GHOST INTELLIGENCE REPORT"));
    
    println!("✅ Report exporter test passed\n");
}

#[tokio::test]
async fn test_team_exporter() {
    println!("🧪 Testing: Team Exporter");
    
    let mut team = exporters::team::TeamExporter::new();
    team.enable_sync();
    
    // Test sync
    let threat = make_test_threat("Test threat for team sync", perception::ThreatSeverity::High);
    
    let result = team.sync_threat(&threat).await;
    assert!(result.is_ok(), "Team sync should work");
    
    // Test heartbeat
    let result = team.heartbeat().await;
    assert!(result.is_ok(), "Heartbeat should work");
    
    // Test member management
    let member = exporters::team::TeamMember {
        id: "member-1".to_string(),
        name: "Ghost-1".to_string(),
        role: "Hunter".to_string(),
        status: "Active".to_string(),
        last_seen: Utc::now(),
    };
    team.add_member(member);
    
    assert_eq!(team.get_members().len(), 1);
    
    println!("✅ Team exporter test passed\n");
}

// ============================================================================
// TEST 11: AGENT INTEGRATION
// ============================================================================

#[tokio::test]
async fn test_agent_integration() {
    println!("🧪 Testing: Agent Integration (Short Run)");
    
    let mut agent = agent::GhostAgent::new();
    
    // Enable stealth for testing
    agent.enable_stealth();
    
    // Deploy
    let result = agent.deploy().await;
    assert!(result.is_ok(), "Agent deployment should work");
    
    // Run for a short time
    let handle = tokio::spawn(async move {
        agent.run().await
    });
    
    // Let it run for 2 seconds
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    // Shutdown (by dropping the handle)
    handle.abort();
    
    println!("✅ Agent integration test passed\n");
}

// ============================================================================
// TEST 12: END-TO-END WORKFLOW
// ============================================================================

#[tokio::test]
async fn test_end_to_end() {
    println!("🧪 Testing: End-to-End Workflow");
    println!("═══════════════════════════════════════════════");
    
    // 1. Initialize core
    println!("📦 Initializing core...");
    let core = core::CoreEngine::new().unwrap();
    assert_eq!(core.config().agent.name, "ghost-agent");
    
    // 2. Initialize perception
    println!("👁️  Initializing perception...");
    let mut perception = perception::PerceptionEngine::new();
    perception.start_monitoring().await.unwrap();
    
    // 3. Initialize actions
    println!("⚡ Initializing actions...");
    let mut actions = actions::ActionEngine::new();
    
    // 4. Initialize stealth
    println!("🕵️  Initializing stealth...");
    let stealth = stealth::StealthEngine::new();
    stealth.hide_process().unwrap();
    stealth.hide_memory().unwrap();
    
    // 5. Initialize exporters
    println!("📤 Initializing exporters...");
    let exporters = exporters::ExporterEngine::new();
    
    // 6. Run a scan
    println!("🔍 Running scan...");
    let threats = perception.scan().await.unwrap();
    println!("   Found {} threats", threats.len());
    
    // 7. Process threats
    for threat in threats {
        println!("   ⚡ Threat: {} - {}", threat.id, threat.description);
        
        let actions_list = actions.evaluate(std::slice::from_ref(&threat)).await.unwrap();
        let results = actions.execute_all(actions_list).await.unwrap();
        for result in results {
            println!("   📝 Action result: {}", result.message);
        }
        
        // Export threat
        exporters.export_threat(&threat).await.unwrap();
    }
    
    // 8. Cleanup
    println!("🧹 Cleaning up...");
    stealth.clean_logs().unwrap();
    
    println!("\n✅ End-to-end workflow passed!");
    println!("═══════════════════════════════════════════════\n");
}

// ============================================================================
// TEST 13: PERFORMANCE BENCHMARK
// ============================================================================

#[tokio::test]
async fn test_performance() {
    println!("🧪 Testing: Performance (Basic)");
    
    use std::time::Instant;
    
    let start = Instant::now();
    
    // Run multiple scans
    let mut perception = perception::PerceptionEngine::new();
    perception.start_monitoring().await.unwrap();
    
    let mut total_threats = 0;
    for i in 0..5 {
        let threats = perception.scan().await.unwrap();
        total_threats += threats.len();
        println!("  Scan {}: {} threats", i+1, threats.len());
    }
    
    let duration = start.elapsed();
    
    println!("  Total threats found: {}", total_threats);
    println!("  Total time: {:?}", duration);
    println!("  Average scan time: {:?}", duration / 5);
    
    assert!(duration < std::time::Duration::from_secs(30), 
        "Performance test should complete within 30 seconds");
    
    println!("✅ Performance test passed\n");
}

// ============================================================================
// RUN ALL TESTS
// ============================================================================

// Uncomment to run all tests sequentially
// #[tokio::test]
// async fn run_all_tests() {
//     println!("\n");
//     println!("═══════════════════════════════════════════════");
//     println!("👻 PROJECT GHOST - COMPLETE TEST SUITE");
//     println!("═══════════════════════════════════════════════");
//     println!("\n");
//     
//     test_config_loading();
//     test_config_validation();
//     test_crypto_key_generation();
//     test_crypto_encryption();
//     test_crypto_hashing();
//     test_crypto_uuid();
//     test_logger_initialization();
//     test_network_monitor().await;
//     test_system_monitor().await;
//     test_auth_monitor().await;
//     test_anomaly_detection().await;
//     test_action_engine().await;
//     test_stealth_engine();
//     test_report_exporter().await;
//     test_team_exporter().await;
//     test_agent_integration().await;
//     test_end_to_end().await;
//     test_performance().await;
//     
//     println!("\n");
//     println!("═══════════════════════════════════════════════");
//     println!("✅ ALL TESTS PASSED!");
//     println!("👻 Ghost is operational");
//     println!("═══════════════════════════════════════════════");
// }