use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::{AppState, Severity};

pub fn draw(frame: &mut Frame<'_>, app: &AppState) {
    if app.is_ghost_mode {
        draw_shadow_dashboard(frame, app);
    } else {
        draw_decoy_tracker(frame, app);
    }
}

fn draw_decoy_tracker(frame: &mut Frame<'_>, app: &AppState) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(8),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new("Smart Standup Assistant")
        .block(Block::default().borders(Borders::ALL).title("Decoy"));
    frame.render_widget(header, vertical[0]);

    let input = Paragraph::new(app.input_buffer.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Scratchpad (Ctrl+A drafts report)"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(input, vertical[1]);

    let output = Paragraph::new(app.standup_output.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("AI Draft Output"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(output, vertical[2]);

    let status = Paragraph::new(format!(
        "{} | Ctrl+G: open dashboard | Ctrl+C: quit",
        app.status_line
    ))
    .block(Block::default().borders(Borders::ALL).title("Status"));
    frame.render_widget(status, vertical[3]);
}

fn draw_shadow_dashboard(frame: &mut Frame<'_>, app: &AppState) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area());

    let context_line = if app.active_contexts.is_empty() {
        "Context: none".to_string()
    } else {
        format!(
            "Context: {} | Tracked ports: {:?}",
            app.active_contexts.join(", "),
            app.tracked_ports
        )
    };

    let header = Paragraph::new(format!("Shadow Dashboard\n{context_line}"))
        .block(Block::default().borders(Borders::ALL).title("Ghost Mode"));
    frame.render_widget(header, vertical[0]);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(64), Constraint::Percentage(36)])
        .split(vertical[1]);

    let mut feed = String::new();
    for line in app.shadow_feed.iter().take(100) {
        feed.push_str(line);
        feed.push('\n');
    }

    if feed.is_empty() {
        feed.push_str("Waiting for system telemetry...");
    }

    let monitor = Paragraph::new(feed)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Sockets + Auth Logs"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(monitor, horizontal[0]);

    let mut anomaly_lines = Vec::new();
    if app.anomaly_feed.is_empty() {
        anomaly_lines.push(Line::from("No context anomalies yet"));
    } else {
        for item in app.anomaly_feed.iter().take(40) {
            let style = match item.severity {
                Severity::Info => Style::default().fg(Color::Cyan),
                Severity::Suspicious => Style::default().fg(Color::Yellow),
                Severity::Critical => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            };
            let tag = match item.severity {
                Severity::Info => "[INFO]",
                Severity::Suspicious => "[SUSPICIOUS]",
                Severity::Critical => "[CRITICAL]",
            };
            anomaly_lines.push(Line::from(vec![
                Span::styled(format!("{tag} "), style),
                Span::raw(item.text.clone()),
            ]));
        }
    }

    let anomalies = Paragraph::new(Text::from(anomaly_lines))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Context-Aware Anomalies"),
        )
        .wrap(Wrap { trim: false });
    frame.render_widget(anomalies, horizontal[1]);

    let footer = Paragraph::new("Esc: panic back to decoy | Ctrl+C: quit")
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    frame.render_widget(footer, vertical[2]);
}
