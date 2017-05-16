use slog::{self, Drain};
use slog_term;
use slog_async;

type DrainType = slog::Fuse<slog_async::Async>;

#[allow(dead_code)]
pub enum LogType {
    Terminal,
    File,
    TerminalAndFile,
}

pub fn create_logger(log_type: LogType) -> slog::Logger {
    use std::sync::Arc;

    let drain = match log_type {
        LogType::Terminal => create_terminal_drain(),
        LogType::File => {
            match create_file_drain() {
                Some(file_drain) => file_drain,
                None => create_null_drain(),
            }
        }
        LogType::TerminalAndFile => create_file_and_terminal_drain(),
    };

    slog::Logger::root(Arc::new(drain), o!("version" => "0.1"))
}

fn create_terminal_drain() -> DrainType {
    //Create basic terminal drain
    let term_decorator = slog_term::TermDecorator::new().build();
    let term_drain = slog_term::CompactFormat::new(term_decorator)
        .build()
        .fuse();

    //Make it async
    slog_async::Async::new(term_drain).build().fuse()
}

// Created on best effort only
fn create_file_drain() -> Option<DrainType> {
    use std::fs::File;

    // Try to create file drain
    match File::create("app.log") {
        Ok(file) => {
            let file_decorator = slog_term::PlainDecorator::new(file);
            let file_drain = slog_term::FullFormat::new(file_decorator).build().fuse();
            let async_file_drain = slog_async::Async::new(file_drain).build().fuse();
            Some(async_file_drain)
        }
        Err(_) => None,
    }
}

fn create_file_and_terminal_drain() -> DrainType {
    let term_drain = create_terminal_drain();

    match create_file_drain() {
        Some(file_drain) => {
            let combined_drain = slog::Duplicate::new(term_drain, file_drain).fuse();
            slog_async::Async::new(combined_drain).build().fuse()
        }
        None => term_drain,
    }
}

fn create_null_drain() -> DrainType {
    let null_drain = slog::Discard;
    slog_async::Async::new(null_drain).build().fuse()
}
