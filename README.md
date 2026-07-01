# 👻 Project GhostShell

**A Silent, Autonomous Defensive Cyber Operations (DCO) Daemon for Linux Infrastructure Protection**

[![Rust](https://img.shields.io/badge/language-Rust_1.70%2B-orange.svg?style=flat-square)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg?style=flat-square)](tests/integration.rs)
[![Mission](https://img.shields.io/badge/mission-Defensive_%2F_Blue_Team-blue.svg?style=flat-square)](#ethical-charter--disclaimer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

---

## 🛡️ Ethical Charter & Disclaimer

**GhostShell is built strictly for defensive cybersecurity, ethical monitoring, and authorized infrastructure protection.**

* **No Black-Hat / Offensive Abuse:** This software is engineered to act as a defensive sentinel on systems you own or are authorized to secure. It is strictly prohibited from being used for unauthorized intrusion, lateral movement, data exfiltration, or malicious attacks.
* **Blue-Team Mission:** The primary objective of GhostShell is to detect intrusions, monitor anomalies, defend infrastructure against exploitation (such as crypto-miners and unauthorized brute-force attempts), and provide tamper-proof intelligence reports to defensive teams.
* **Ethics First:** By deploying GhostShell, you commit to utilizing its capabilities in full compliance with applicable laws, ethical standards, and rules of engagement.

---

## 🧠 System Architecture

GhostShell replaces traditional interactive terminal dashboards with a **silent, background service architecture** designed for high-throughput monitoring and rapid defensive countermeasures.

```
       +-------------------------------------------------------+
       |                  GHOSTSHELL DAEMON                    |
       |                                                       |
       |  +-------------------------------------------------+  |
       |  |               PERCEPTION ENGINE                 |  |
       |  |  [Network Monitor] [System Monitor] [Auth Logs] |  |
       |  |              \            |            /        |  |
       |  |               v           v           v         |  |
       |  |              [ Anomaly Detector & Intelligence] |  |
       |  +-------------------------------------------------+  |
       |                           |                           |
       |                           v (Threat Slices)           |
       |  +-------------------------------------------------+  |
       |  |                 ACTION ENGINE                   |  |
       |  |  [Evaluate Threats] ---> [Execute Countermeasures] |  |
       |  +-------------------------------------------------+  |
       |                           |                           |
       |             +-------------+-------------+             |
       |             |                           |             |
       |             v                           v             |
       |  +---------------------+     +---------------------+  |
       |  |   STEALTH ENGINE    |     |   EXPORTER ENGINE   |  |
       |  | [Process/Mem Guard] |     | [Team Sync & Intel] |  |
       |  +---------------------+     +---------------------+  |
       +-------------------------------------------------------+
```

### 1. 👁️ Perception Engine (The Senses)
GhostShell continuously monitors local system telemetry without relying on third-party user interfaces:
* **Network Monitor:** Analyzes active TCP/UDP socket states, tracks listening ports, and flags unauthorized external connections.
* **System Monitor:** Watches running process tables, CPU/memory consumption, and process lineage (e.g., catching suspicious parent-child spawns like `sh` from web servers).
* **Auth Monitor:** Audits authentication events to detect brute-force login attempts and privilege escalation.
* **Anomaly Detector & Threat Intelligence:** Fuses multi-source telemetry against baseline profiles to immediately identify known malicious utilities and mining payloads (such as `xmrig`, `netcat`, `nmap`, and `mimikatz`).

### 2. ⚡ Action Engine (Defensive Countermeasures)
When anomalies cross the defined risk threshold, the Action Engine evaluates threat batches asynchronously and deploys protective countermeasures:
* **Neutralize / Eliminate:** Autonomously terminates suspicious or malicious processes attempting to abuse system resources.
* **Counter Strategy:** Adapts defensive posture based on threat severity (e.g., active connection disruption or passive probing).
* **Deception:** Configurable deception campaigns to mislead intruders and gather defensive telemetry.

### 3. 🕵️ Stealth Engine (Self-Protection)
To prevent malware or unauthorized intruders from disabling the security monitor, GhostShell incorporates self-protection capabilities:
* **Process & Memory Obfuscation:** Masks security daemon footprints from basic user-space enumeration.
* **Log Cleanups & Secure Wiping:** Ensures sensitive operational telemetry doesn't leave unencrypted traces on local disk.

### 4. 📤 Exporter Engine (Intelligence & Collaboration)
* **Encrypted Reporting:** Generates structured, tamper-proof `IntelligenceReport` artifacts encrypted via `AES-256-GCM`.
* **Team Synchronization:** Broadcasts covert heartbeats and syncs threat telemetry with remote blue-team coordination servers.

---

## 🚀 Installation & Setup

### Prerequisites
* **OS:** Linux (Ubuntu/Debian, Arch, RHEL, etc.)
* **Toolchain:** [Rust 1.70+](https://rustup.rs/) (`cargo`, `rustc`)

### Quick Install
You can use the included installation script to build and install GhostShell system-wide:

```bash
chmod +x install.sh
./install.sh
```

### Manual Build & Installation
If you prefer to build manually from source:

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/GhostShell.git
cd GhostShell

# 2. Build in release mode
cargo build --release

# 3. Install binary to PATH
sudo cp target/release/project-ghost /usr/local/bin/ghost

# 4. Setup configuration
sudo mkdir -p /etc/ghost
sudo cp config/ghost.yaml /etc/ghost/
```

---

## 💻 Usage & Deployment

GhostShell operates non-interactively as a background daemon.

### Deploying the Daemon
To start the daemon in autonomous monitoring mode with stealth protection enabled:

```bash
ghost --deploy --stealth
```

### Command-Line Arguments
```
Usage: ghost [OPTIONS]

Options:
  -d, --deploy           Deploy and start the background monitoring daemon
  -s, --stealth          Enable stealth engine self-protection capabilities
  -c, --config <CONFIG>  Specify custom configuration file path (default: /etc/ghost/ghost.yaml or ~/.config/ghost/config.yaml)
  -h, --help             Print help information
  -V, --version          Print version information
```

---

## ⚙️ Configuration (`ghost.yaml`)

GhostShell is fully configured via YAML. The configuration loader checks `./config/ghost.yaml`, `/etc/ghost/config.yaml`, and `~/.config/ghost/config.yaml`.

```yaml
agent:
  name: "ghost-agent"
  version: "0.1.0"
  autonomous: true
  scan_interval: 5               # Telemetry scan frequency in seconds
  self_destruct_on_detect: true  # Wipe memory if tampering is detected
  max_concurrent_ops: 10

perception:
  network: true
  system: true
  auth: true
  filesystem: true
  scan_interval: 5
  anomaly_threshold: 0.7         # Confidence threshold to trigger defensive actions
  max_history: 10000

actions:
  auto_neutralize: true          # Automatically terminate malicious payloads
  probe_depth: "aggressive"
  counter_strategy: "adaptive"
  deception_enabled: true
  max_actions_per_scan: 10

stealth:
  hide_process: true
  hide_memory: true
  obfuscate_traffic: true
  clean_logs: true
  self_destruct_on_detect: true
  encryption_level: "military"

reporting:
  method: "covert"
  encryption: "AES-256-GCM"
  destination: "https://intel.ghost.local/report"
  frequency: 60
  compression: true

security:
  encryption_key_rotation: 3600
  max_failures: 5
  integrity_check: true
```

---

## 🧪 Testing & Verification

GhostShell includes a comprehensive integration test suite that simulates cyber attacks, tests cryptographic integrity, verifies anomaly detection, and benchmarks batch action execution.

To run the full verification suite:

```bash
# Run all unit and integration tests (23 tests total)
cargo test

# Run specifically the integration test workflow
cargo test --test integration_test
```

### What the Test Suite Verifies:
1. **Configuration Integrity:** Validates YAML loading, default fallback, and schema validation.
2. **Cryptographic Security:** Tests SHA-256 hashing, AES-256-GCM encryption/decryption, secure memory zeroization, and UUID generation.
3. **Perception & Anomaly Detection:** Simulates network socket tables, authentication events, and process trees (asserting positive detection of tools like `xmrig` and `mimikatz`).
4. **Action & Stealth Engines:** Verifies batch threat evaluation, action dispatching, memory hiding, and log sanitization.
5. **Exporter Engine:** Asserts valid JSON serialization and team synchronization handshakes.

---

## 🤝 Contributing

Contributions are welcome from security researchers, systems engineers, and blue-team developers!
1. Fork the repository.
2. Create a feature branch (`git checkout -b feat/defensive-enhancement`).
3. Ensure all tests pass (`cargo test`).
4. Commit your changes modularly with descriptive conventional commits.
5. Open a Pull Request.

---

## 📄 License

This project is open-source and licensed under the **MIT License**. See `LICENSE` for details.
