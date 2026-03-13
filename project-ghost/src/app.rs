use std::collections::{BTreeSet, VecDeque};

#[derive(Clone, Debug)]
pub struct SocketRecord {
    pub protocol: String,
    pub state: String,
    pub local_addr: String,
    pub local_port: Option<u16>,
    pub peer_addr: String,
    pub peer_port: Option<u16>,
    pub process: Option<String>,
}

pub enum AppEvent {
    AiDraftReady(Result<String, String>),
    SocketTelemetry {
        summary: String,
        records: Vec<SocketRecord>,
    },
    AuthLogSnapshot(String),
    MonitorError(String),
}

pub struct AppState {
    pub is_ghost_mode: bool,
    pub should_quit: bool,
    pub input_buffer: String,
    pub standup_output: String,
    pub shadow_feed: VecDeque<String>,
    pub anomaly_feed: VecDeque<Anomaly>,
    pub active_contexts: Vec<String>,
    pub tracked_ports: Vec<u16>,
    pub status_line: String,
}

#[derive(Clone, Debug)]
pub enum Severity {
    Info,
    Suspicious,
    Critical,
}

#[derive(Clone, Debug)]
pub struct Anomaly {
    pub severity: Severity,
    pub text: String,
}

#[derive(Clone, Copy)]
struct ProjectProfile {
    name: &'static str,
    keywords: &'static [&'static str],
    ports: &'static [u16],
}

impl AppState {
    pub fn new() -> Self {
        Self {
            is_ghost_mode: false,
            should_quit: false,
            input_buffer: String::new(),
            standup_output: String::new(),
            shadow_feed: VecDeque::with_capacity(256),
            anomaly_feed: VecDeque::with_capacity(128),
            active_contexts: Vec::new(),
            tracked_ports: Vec::new(),
            status_line: "Ready".to_string(),
        }
    }

    pub fn refresh_context_from_notes(&mut self) {
        let lowered = self.input_buffer.to_lowercase();
        let mut context_set = BTreeSet::new();
        let mut port_set = BTreeSet::new();

        for profile in project_profiles() {
            if profile
                .keywords
                .iter()
                .any(|keyword| lowered.contains(keyword))
            {
                context_set.insert(profile.name.to_string());
                for port in profile.ports {
                    port_set.insert(*port);
                }
            }
        }

        self.active_contexts = context_set.into_iter().collect();
        self.tracked_ports = port_set.into_iter().collect();
    }

    pub fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::AiDraftReady(Ok(text)) => {
                self.standup_output = text;
                self.status_line = "Standup draft ready".to_string();
            }
            AppEvent::AiDraftReady(Err(err)) => {
                self.status_line = format!("AI draft failed: {err}");
            }
            AppEvent::SocketTelemetry { summary, records } => {
                for line in summary.lines() {
                    self.push_shadow_line(format!("[SOCKET] {line}"));
                }
                self.score_socket_records(records);
            }
            AppEvent::AuthLogSnapshot(lines) => {
                for line in lines.lines() {
                    self.push_shadow_line(format!("[AUTH] {line}"));
                }
            }
            AppEvent::MonitorError(err) => {
                self.push_shadow_line(format!("[ERROR] {err}"));
            }
        }
    }

    fn score_socket_records(&mut self, records: Vec<SocketRecord>) {
        if self.tracked_ports.is_empty() {
            return;
        }

        for rec in records {
            let Some(local_port) = rec.local_port else {
                continue;
            };

            if !self.tracked_ports.contains(&local_port) {
                continue;
            }

            let listen_state = rec.state.eq_ignore_ascii_case("LISTEN")
                || rec.state.eq_ignore_ascii_case("UNCONN");

            if listen_state && is_exposed_addr(&rec.local_addr) {
                self.push_anomaly(Severity::Critical, format!(
                    "{} context port {local_port} exposed on {} ({})",
                    rec.protocol,
                    rec.local_addr,
                    process_label(&rec)
                ));
                continue;
            }

            if let Some(peer_port) = rec.peer_port {
                if !is_internal_addr(&rec.peer_addr) && peer_port != 0 {
                    self.push_anomaly(Severity::Suspicious, format!(
                        "{} context port {local_port} talks to external {}:{} ({})",
                        rec.protocol,
                        rec.peer_addr,
                        peer_port,
                        process_label(&rec)
                    ));
                    continue;
                }
            }

            self.push_anomaly(
                Severity::Info,
                format!(
                    "{} context port {local_port} observed in {} state ({})",
                    rec.protocol,
                    rec.state,
                    process_label(&rec)
                ),
            );
        }
    }

    fn push_shadow_line(&mut self, line: String) {
        const MAX_LINES: usize = 260;
        self.shadow_feed.push_front(line);
        while self.shadow_feed.len() > MAX_LINES {
            let _ = self.shadow_feed.pop_back();
        }
    }

    fn push_anomaly(&mut self, severity: Severity, text: String) {
        const MAX_LINES: usize = 140;
        self.anomaly_feed.push_front(Anomaly { severity, text });
        while self.anomaly_feed.len() > MAX_LINES {
            let _ = self.anomaly_feed.pop_back();
        }
    }
}

fn process_label(rec: &SocketRecord) -> String {
    rec.process
        .as_ref()
        .map_or_else(|| "unknown-process".to_string(), |s| s.clone())
}

fn is_exposed_addr(addr: &str) -> bool {
    let stripped = strip_ipv6_prefix(addr);
    stripped == "0.0.0.0" || stripped == "::" || stripped == "*"
}

fn is_internal_addr(addr: &str) -> bool {
    let stripped = strip_ipv6_prefix(addr);
    if stripped == "*"
        || stripped == "0.0.0.0"
        || stripped == "::"
        || stripped.starts_with("127.")
        || stripped == "localhost"
        || stripped == "::1"
        || stripped.starts_with("10.")
        || stripped.starts_with("192.168.")
    {
        return true;
    }

    if let Some((a, b, _, _)) = parse_ipv4_octets(stripped) {
        return a == 172 && (16..=31).contains(&b);
    }

    false
}

fn strip_ipv6_prefix(addr: &str) -> &str {
    addr.trim_matches('[')
        .trim_matches(']')
        .split('%')
        .next()
        .unwrap_or(addr)
}

fn parse_ipv4_octets(addr: &str) -> Option<(u8, u8, u8, u8)> {
    let mut parts = addr.split('.');
    let a = parts.next()?.parse::<u8>().ok()?;
    let b = parts.next()?.parse::<u8>().ok()?;
    let c = parts.next()?.parse::<u8>().ok()?;
    let d = parts.next()?.parse::<u8>().ok()?;
    Some((a, b, c, d))
}

fn project_profiles() -> &'static [ProjectProfile] {
    &[
        ProjectProfile {
            name: "Project Mercury",
            keywords: &["project mercury", "mercury", "mercury-api", "mercury service"],
            ports: &[8443, 9000, 50051],
        },
        ProjectProfile {
            name: "Project Atlas",
            keywords: &["project atlas", "atlas", "atlas edge"],
            ports: &[443, 9443, 8080],
        },
        ProjectProfile {
            name: "Project Orion",
            keywords: &["project orion", "orion", "orion relay"],
            ports: &[22, 2222, 3389],
        },
    ]
}
