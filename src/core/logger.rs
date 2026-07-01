//! Logging Engine - Ghost's Silent Observer
//!
//! Provides:
//! - Structured logging
//! - Multiple log levels
//! - File and console output
//! - Log rotation
//! - JSON formatting

use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::{
    fmt::{self, time::UtcTime},
    prelude::*,
    EnvFilter,
    Layer,
};

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub console: bool,
    pub file: bool,
    pub file_path: String,
    pub json_format: bool,
    pub include_timestamp: bool,
}

/// Initialize logging system
pub fn init() {
    init_with_config(LoggerConfig::default());
}

/// Initialize logging with custom configuration
pub fn init_with_config(config: LoggerConfig) {
    let level = match config.level {
        LogLevel::Trace => Level::TRACE,
        LogLevel::Debug => Level::DEBUG,
        LogLevel::Info => Level::INFO,
        LogLevel::Warn => Level::WARN,
        LogLevel::Error => Level::ERROR,
    };

    let filter = EnvFilter::from_default_env()
        .add_directive(level.into());

    let mut layers = Vec::new();

    // Console layer
    if config.console {
        let console_layer = fmt::Layer::new()
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
            .with_line_number(false)
            .with_ansi(true);

        layers.push(console_layer.boxed());
    }

    // File layer
    if config.file {
        if let Ok(file) = std::fs::File::create(&config.file_path) {
            let file_layer = fmt::Layer::new()
                .with_writer(std::sync::Arc::new(file))
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(false)
                .with_line_number(false)
                .with_ansi(false)
                .with_timer(UtcTime::rfc_3339());

            layers.push(file_layer.boxed());
        }
    }

    // Build subscriber
    let subscriber = tracing_subscriber::Registry::default()
        .with(filter)
        .with(layers);

    let _ = tracing::subscriber::set_global_default(subscriber);
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            console: true,
            file: true,
            file_path: "ghost.log".to_string(),
            json_format: false,
            include_timestamp: true,
        }
    }
}

/// Log a message with structured context
pub fn log_with_context(
    level: LogLevel,
    message: &str,
    context: &[(&str, &str)],
) {
    let context_str = context
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join(" ");

    let full_message = if context.is_empty() {
        message.to_string()
    } else {
        format!("{} {}", message, context_str)
    };

    match level {
        LogLevel::Trace => trace!("{}", full_message),
        LogLevel::Debug => debug!("{}", full_message),
        LogLevel::Info => info!("{}", full_message),
        LogLevel::Warn => warn!("{}", full_message),
        LogLevel::Error => error!("{}", full_message),
    }
}

/// Create a log file with rotation
pub fn rotate_log_file(path: &str, max_size: u64) -> Result<(), std::io::Error> {
    let metadata = std::fs::metadata(path)?;
    if metadata.len() > max_size {
        // Rotate the log
        let backup_path = format!("{}.old", path);
        if std::fs::rename(path, &backup_path).is_err() {
            // If rename fails, just truncate
            let _ = std::fs::File::create(path)?;
        }
    }
    Ok(())
}

/// Log macro with context
#[macro_export]
macro_rules! log_info {
    ($msg:expr, $($key:expr => $value:expr),*) => {
        $crate::core::logger::log_with_context(
            $crate::core::logger::LogLevel::Info,
            $msg,
            &[$(($key, $value)),*]
        );
    };
}

#[macro_export]
macro_rules! log_warn {
    ($msg:expr, $($key:expr => $value:expr),*) => {
        $crate::core::logger::log_with_context(
            $crate::core::logger::LogLevel::Warn,
            $msg,
            &[$(($key, $value)),*]
        );
    };
}

#[macro_export]
macro_rules! log_error {
    ($msg:expr, $($key:expr => $value:expr),*) => {
        $crate::core::logger::log_with_context(
            $crate::core::logger::LogLevel::Error,
            $msg,
            &[$(($key, $value)),*]
        );
    };
}