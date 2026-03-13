mod app;
mod tasks;
mod ui;

use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc;

use app::{AppEvent, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = AppState::new();
    if let Ok(Some(notes)) = tasks::load_notes().await {
        app.input_buffer = notes;
        app.refresh_context_from_notes();
    }

    let (tx, mut rx) = mpsc::channel::<AppEvent>(256);

    tasks::spawn_background_monitors(tx.clone());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let run_result = async {
        while !app.should_quit {
            while let Ok(event) = rx.try_recv() {
                app.apply_event(event);
            }

            terminal.draw(|frame| ui::draw(frame, &app))?;

            if event::poll(Duration::from_millis(33))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match (key.modifiers, key.code) {
                            (KeyModifiers::CONTROL, KeyCode::Char('c')) => app.should_quit = true,
                            (KeyModifiers::CONTROL, KeyCode::Char('g')) => {
                                app.is_ghost_mode = true;
                                app.status_line = "Shadow Dashboard active".to_string();
                            }
                            (_, KeyCode::Esc) if app.is_ghost_mode => {
                                app.is_ghost_mode = false;
                                app.status_line = "Returned to Standup Assistant".to_string();
                            }
                            (KeyModifiers::CONTROL, KeyCode::Char('a')) if !app.is_ghost_mode => {
                                app.status_line = "Drafting standup update...".to_string();
                                let prompt = app.input_buffer.clone();
                                let tx_clone = tx.clone();
                                tokio::spawn(async move {
                                    let result = tasks::draft_standup_with_llm(prompt)
                                        .await
                                        .map_err(|err| err.to_string());
                                    let _ = tx_clone.send(AppEvent::AiDraftReady(result)).await;
                                });
                            }
                            (_, KeyCode::Backspace) if !app.is_ghost_mode => {
                                app.input_buffer.pop();
                                app.refresh_context_from_notes();
                                tasks::spawn_note_save(app.input_buffer.clone());
                            }
                            (_, KeyCode::Enter) if !app.is_ghost_mode => {
                                app.input_buffer.push('\n');
                                app.refresh_context_from_notes();
                                tasks::spawn_note_save(app.input_buffer.clone());
                            }
                            (_, KeyCode::Char(ch)) if !app.is_ghost_mode => {
                                app.input_buffer.push(ch);
                                app.refresh_context_from_notes();
                                tasks::spawn_note_save(app.input_buffer.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Ok::<(), anyhow::Error>(())
    }
    .await;

    let mut teardown_error: Option<io::Error> = None;
    if let Err(err) = disable_raw_mode() {
        teardown_error = Some(err);
    }
    if let Err(err) = execute!(terminal.backend_mut(), LeaveAlternateScreen) {
        teardown_error = Some(err);
    }
    if let Err(err) = terminal.show_cursor() {
        teardown_error = Some(err);
    }

    run_result?;
    if let Some(err) = teardown_error {
        return Err(err.into());
    }

    Ok(())
}
