// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Captures ERROR-level tracing events in memory for integration tests.
//!
//! The integration suite drives real HTTP requests against the full app. When a
//! handler returns `500 ExceptionInternal`, the underlying cause (usually an SQL
//! error swallowed by `AppError::Internal`) is only emitted through `tracing`.
//! Without a subscriber installed in the test process that information is lost,
//! leaving a bare "500" in the panic output.
//!
//! This module installs a process-wide subscriber that records ERROR events
//! (formatted, ANSI-free) into a bounded in-memory buffer, plus a panic hook that
//! dumps the buffer next to the failing assertion. Cargo's test harness prints
//! the panic message for failing tests, so the captured cause travels with it.

use std::io::Write;
use std::sync::{Mutex, Once, OnceLock};

/// Keep the most recent ERROR lines; enough to cover one failing scenario
/// without letting a long run grow unbounded.
const MAX_LINES: usize = 200;

fn buffer() -> &'static Mutex<Vec<String>> {
    static BUFFER: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
    BUFFER.get_or_init(|| Mutex::new(Vec::new()))
}

fn record(line: &str) {
    let line = line.trim_end();
    if line.is_empty() {
        return;
    }
    let mut buf = buffer().lock().unwrap_or_else(|e| e.into_inner());
    buf.push(line.to_string());
    if buf.len() > MAX_LINES {
        let excess = buf.len() - MAX_LINES;
        buf.drain(0..excess);
    }
}

struct CaptureWriter;

impl Write for CaptureWriter {
    fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
        for line in String::from_utf8_lossy(data).split('\n') {
            record(line);
        }
        Ok(data.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn dump() {
    let buf = buffer().lock().unwrap_or_else(|e| e.into_inner());
    if buf.is_empty() {
        return;
    }
    eprintln!("--- captured ERROR logs (last {} lines) ---", buf.len());
    for line in buf.iter() {
        eprintln!("{line}");
    }
    eprintln!("--- end captured ERROR logs ---");
}

/// Drain the buffered ERROR lines.
///
/// Used by tests that need **per-request** attribution (e.g. the seeded route
/// scan records which SQL error belongs to which route) instead of a single
/// dump at the end of the run.
#[allow(dead_code)] // 供按需逐请求归因；integration_runner 自身仍只用末尾 dump
pub fn drain() -> Vec<String> {
    let mut buf = buffer().lock().unwrap_or_else(|e| e.into_inner());
    std::mem::take(&mut *buf)
}

/// Install the capturing subscriber and panic hook. Idempotent.
///
/// Safe to call from any test setup path; only the first call takes effect.
pub fn init() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::ERROR)
            .with_ansi(false)
            .with_writer(|| CaptureWriter)
            .finish();
        // Another subscriber may already be installed (e.g. by a test that
        // configures its own); the panic hook still dumps whatever was captured.
        let _ = tracing::subscriber::set_global_default(subscriber);

        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            dump();
            previous(info);
        }));
    });
}
