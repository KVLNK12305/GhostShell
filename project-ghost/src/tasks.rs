use std::process::Stdio;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use chrono::Local;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::process::Command;
use tokio::sync::mpsc;

use crate::app::{AppEvent, SocketRecord};

const NOTES_FILE: &str = ".ghost_notes.json";

#[derive(Debug, Serialize, Deserialize)]
struct NotePayload {
    notes: String,
}

pub fn spawn_background_monitors(tx: mpsc::Sender<AppEvent>) {
    let tx_sockets = tx.clone();
    tokio::spawn(async move {
        stream_socket_snapshots(tx_sockets).await;
    });

    tokio::spawn(async move {
        stream_auth_log_snapshots(tx).await;
    });
}

pub fn spawn_note_save(notes: String) {
    tokio::spawn(async move {
        let _ = save_notes(notes).await;
    });
}

pub async fn save_notes(notes: String) -> Result<()> {
    let payload = NotePayload { notes };
    let serialized = serde_json::to_vec(&payload).context("serialize notes")?;
    fs::write(NOTES_FILE, serialized)
        .await
        .context("write .ghost_notes.json")?;
    Ok(())
}

pub async fn load_notes() -> Result<Option<String>> {
    match fs::read(NOTES_FILE).await {
        Ok(bytes) => {
            let payload: NotePayload =
                serde_json::from_slice(&bytes).context("parse .ghost_notes.json")?;
            Ok(Some(payload.notes))
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err).context("read .ghost_notes.json"),
    }
}

pub async fn draft_standup_with_llm(raw_notes: String) -> Result<String> {
    let api_url = std::env::var("GHOST_LLM_URL").ok();
    let api_key = std::env::var("GHOST_LLM_KEY").ok();

    if let (Some(url), Some(key)) = (api_url, api_key) {
        return call_remote_llm(&url, &key, raw_notes).await;
    }

    // Fallback keeps the UI behavior usable before API credentials are configured.
    tokio::time::sleep(Duration::from_millis(500)).await;
    Ok(format!(
        "Yesterday:\n- Continued Project Ghost development\n\nToday:\n- Integrate async AI provider\n\nBlockers:\n- API credentials not configured\n\nRaw Notes:\n{}",
        raw_notes.trim()
    ))
}

async fn call_remote_llm(url: &str, api_key: &str, raw_notes: String) -> Result<String> {
    let prompt = format!(
        "Convert these rough standup notes into a concise professional daily update with sections: Yesterday, Today, Blockers. Notes:\n{}",
        raw_notes
    );

    let body = serde_json::json!({
        "model": "gpt-4.1-mini",
        "input": prompt
    });

    let client = Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .context("send LLM request")?;

    if !response.status().is_success() {
        let status = response.status();
        let body_text = response.text().await.unwrap_or_else(|_| "<no body>".to_string());
        return Err(anyhow!("LLM request failed ({status}): {body_text}"));
    }

    let json_value: serde_json::Value = response.json().await.context("decode LLM response")?;

    let text = json_value
        .get("output_text")
        .and_then(|v| v.as_str())
        .map(ToString::to_string)
        .or_else(|| {
            json_value
                .get("choices")
                .and_then(|choices| choices.get(0))
                .and_then(|first| first.get("message"))
                .and_then(|msg| msg.get("content"))
                .and_then(|content| {
                    content
                        .as_array()
                        .and_then(|parts| parts.first())
                        .and_then(|part| part.get("text"))
                        .and_then(|text| text.as_str())
                })
                .map(ToString::to_string)
        })
        .ok_or_else(|| anyhow!("Could not find model text in response"))?;

    Ok(text)
}

async fn stream_socket_snapshots(tx: mpsc::Sender<AppEvent>) {
    loop {
        let output = if cfg!(target_os = "windows") {
            Command::new("netstat")
                .args(["-ano"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("ss -tulnp 2>/dev/null || netstat -tulnp 2>/dev/null")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
        };

        match output {
            Ok(out) => {
                let snapshot = String::from_utf8_lossy(&out.stdout);
                let preview: String = snapshot.lines().take(12).collect::<Vec<_>>().join("\n");
                let records = parse_socket_snapshot(&snapshot);
                let line = format!(
                    "{}\n{}",
                    Local::now().format("%H:%M:%S socket snapshot"),
                    preview
                );
                let _ = tx
                    .send(AppEvent::SocketTelemetry {
                        summary: line,
                        records,
                    })
                    .await;
            }
            Err(err) => {
                let _ = tx
                    .send(AppEvent::MonitorError(format!("socket monitor: {err}")))
                    .await;
            }
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

fn parse_socket_snapshot(raw: &str) -> Vec<SocketRecord> {
    let mut records = Vec::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("Netid") || trimmed.starts_with("Proto") {
            continue;
        }

        if let Some(record) = parse_ss_line(trimmed) {
            records.push(record);
            continue;
        }

        if let Some(record) = parse_netstat_line(trimmed) {
            records.push(record);
        }
    }

    records
}

fn parse_ss_line(line: &str) -> Option<SocketRecord> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 6 {
        return None;
    }
    if parts[1].bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }

    let protocol = parts[0].to_string();
    let state = parts[1].to_string();
    let local = parts[4];
    let peer = parts[5];
    let process = if parts.len() > 6 {
        Some(parts[6..].join(" "))
    } else {
        None
    };

    let (local_addr, local_port) = parse_endpoint(local);
    let (peer_addr, peer_port) = parse_endpoint(peer);

    Some(SocketRecord {
        protocol,
        state,
        local_addr,
        local_port,
        peer_addr,
        peer_port,
        process,
    })
}

fn parse_netstat_line(line: &str) -> Option<SocketRecord> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 5 {
        return None;
    }

    let protocol = parts[0].to_string();
    let local = parts[3];
    let peer = parts[4];

    let state = if parts.len() > 5 {
        parts[5].to_string()
    } else {
        "UNKNOWN".to_string()
    };

    let process = if parts.len() > 6 {
        Some(parts[6..].join(" "))
    } else {
        None
    };

    let (local_addr, local_port) = parse_endpoint(local);
    let (peer_addr, peer_port) = parse_endpoint(peer);

    Some(SocketRecord {
        protocol,
        state,
        local_addr,
        local_port,
        peer_addr,
        peer_port,
        process,
    })
}

fn parse_endpoint(endpoint: &str) -> (String, Option<u16>) {
    let cleaned = endpoint.trim().trim_matches('[').trim_matches(']');
    if cleaned.is_empty() {
        return ("unknown".to_string(), None);
    }

    if cleaned == "*" {
        return ("*".to_string(), None);
    }

    if let Some((addr, port_str)) = cleaned.rsplit_once(':') {
        let port = port_str.parse::<u16>().ok();
        return (addr.to_string(), port);
    }

    (cleaned.to_string(), None)
}

async fn stream_auth_log_snapshots(tx: mpsc::Sender<AppEvent>) {
    loop {
        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args([
                    "-NoProfile",
                    "-Command",
                    "Get-WinEvent -LogName Security -MaxEvents 8 | Select-Object -ExpandProperty Message",
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("tail -n 8 /var/log/auth.log 2>/dev/null || tail -n 8 /var/log/secure 2>/dev/null")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
        };

        match output {
            Ok(out) => {
                let snapshot = String::from_utf8_lossy(&out.stdout);
                let line = format!(
                    "{}\n{}",
                    Local::now().format("%H:%M:%S auth snapshot"),
                    snapshot
                );
                let _ = tx.send(AppEvent::AuthLogSnapshot(line)).await;
            }
            Err(err) => {
                let _ = tx
                    .send(AppEvent::MonitorError(format!("auth monitor: {err}")))
                    .await;
            }
        }

        tokio::time::sleep(Duration::from_secs(6)).await;
    }
}
