use std::io::{stderr, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{Attribute, Print, SetAttribute},
    terminal::{self, ClearType},
};

use crate::db::Entry;
use crate::matcher;

const MAX_DISPLAY: usize = 10;

pub fn run(all_entries: Vec<&Entry>, initial_query: String) -> Option<String> {
    // Pre-filter with initial query; skip TUI if result is unambiguous
    let initial_filtered = apply_filter(&all_entries, &initial_query);
    if initial_filtered.is_empty() {
        return None;
    }
    if initial_filtered.len() == 1 {
        return Some(initial_filtered[0].command.clone());
    }

    let mut query = initial_query;
    let mut selected: usize = 0;
    let mut err = stderr();

    terminal::enable_raw_mode().ok()?;
    execute!(err, cursor::Hide).ok();

    let result = event_loop(&mut err, &all_entries, &mut query, &mut selected);

    terminal::disable_raw_mode().ok();
    execute!(err, cursor::Show).ok();

    result
}

fn apply_filter<'a>(all_entries: &[&'a Entry], query: &str) -> Vec<&'a Entry> {
    let keywords: Vec<String> = query
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    matcher::filter(all_entries.to_vec(), &keywords)
}

fn draw(err: &mut impl Write, query: &str, filtered: &[&Entry], selected: usize) -> usize {
    let display_count = filtered.len().min(MAX_DISPLAY);
    let extra = filtered.len().saturating_sub(MAX_DISPLAY);

    queue!(err, Print(format!("> {}\r\n", query))).ok();

    for (i, entry) in filtered.iter().take(display_count).enumerate() {
        if i == selected {
            queue!(
                err,
                SetAttribute(Attribute::Reverse),
                Print(format!("  {}\r\n", entry.command)),
                SetAttribute(Attribute::Reset),
            )
            .ok();
        } else {
            queue!(err, Print(format!("  {}\r\n", entry.command))).ok();
        }
    }

    let mut lines = 1 + display_count;
    if extra > 0 {
        queue!(err, Print(format!("  他{}件\r\n", extra))).ok();
        lines += 1;
    }

    err.flush().ok();
    lines
}

fn clear_ui(err: &mut impl Write, drawn_lines: usize) {
    if drawn_lines > 0 {
        execute!(
            err,
            cursor::MoveUp(drawn_lines as u16),
            terminal::Clear(ClearType::FromCursorDown),
        )
        .ok();
    }
}

fn event_loop(
    err: &mut std::io::Stderr,
    all_entries: &[&Entry],
    query: &mut String,
    selected: &mut usize,
) -> Option<String> {
    let mut drawn_lines: usize = 0;

    loop {
        let filtered = apply_filter(all_entries, query);

        // Clamp selection to visible range
        let max_sel = filtered.len().min(MAX_DISPLAY).saturating_sub(1);
        if *selected > max_sel {
            *selected = max_sel;
        }

        clear_ui(err, drawn_lines);
        drawn_lines = draw(err, query, &filtered, *selected);

        match event::read().ok()? {
            Event::Key(key) => match (key.code, key.modifiers) {
                // Cancel
                (KeyCode::Char('c'), KeyModifiers::CONTROL)
                | (KeyCode::Char('q'), KeyModifiers::NONE)
                | (KeyCode::Esc, _) => {
                    clear_ui(err, drawn_lines);
                    return None;
                }
                // Confirm
                (KeyCode::Enter, _) => {
                    let result = filtered.get(*selected).map(|e| e.command.clone());
                    clear_ui(err, drawn_lines);
                    return result;
                }
                // Move down
                (KeyCode::Char('j'), KeyModifiers::NONE)
                | (KeyCode::Down, _)
                | (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                    if *selected < filtered.len().min(MAX_DISPLAY).saturating_sub(1) {
                        *selected += 1;
                    }
                }
                // Move up
                (KeyCode::Char('k'), KeyModifiers::NONE)
                | (KeyCode::Up, _)
                | (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                    if *selected > 0 {
                        *selected -= 1;
                    }
                }
                // Top
                (KeyCode::Char('g'), KeyModifiers::NONE) => {
                    *selected = 0;
                }
                // Bottom
                (KeyCode::Char('G'), _) => {
                    *selected = filtered.len().min(MAX_DISPLAY).saturating_sub(1);
                }
                // Delete char from query
                (KeyCode::Backspace, _) => {
                    query.pop();
                    *selected = 0;
                }
                // Append to query (j/k/g/q reserved above, won't reach here)
                (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    query.push(c);
                    *selected = 0;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
