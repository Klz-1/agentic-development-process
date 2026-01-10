//! Status bar - displays context info and key hints
//!
//! A polished status bar showing build status, model info, current workspace,
//! and contextual keyboard shortcuts.

use crate::app::{App, FocusedPanel};
use crate::tmux::SessionStatus;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct StatusBar<'a> {
    app: &'a App,
}

impl<'a> StatusBar<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        // Left section: Build status and model info
        let mut left_spans: Vec<Span> = Vec::new();

        // Build/status indicator based on current session
        let (status_text, status_style) = self.get_build_status();
        left_spans.push(Span::styled(
            format!(" {} ", status_text),
            status_style,
        ));

        // Model info (shown as green label)
        left_spans.push(Span::styled(
            " Opus 4.5 ",
            Style::default().fg(Color::Green),
        ));
        left_spans.push(Span::styled(
            "Claude Code ",
            Style::default().fg(Color::Gray),
        ));

        // Right section: Current workspace and key hints
        let mut right_spans: Vec<Span> = Vec::new();

        // Current workspace/session name
        if let Some(session) = self.app.selected_session_data() {
            right_spans.push(Span::styled(
                session.name.clone(),
                Style::default().fg(Color::Cyan),
            ));
        }

        // Separator
        right_spans.push(Span::styled("  ", Style::default()));

        // Key hints - more compact and polished
        let hints = self.get_key_hints();
        for (i, (key, action)) in hints.iter().enumerate() {
            right_spans.push(Span::styled(
                (*key).to_string(),
                Style::default().fg(Color::Yellow),
            ));
            right_spans.push(Span::styled(
                format!(" {}", action),
                Style::default().fg(Color::DarkGray),
            ));
            if i < hints.len() - 1 {
                right_spans.push(Span::styled("  ", Style::default()));
            }
        }
        right_spans.push(Span::raw(" "));

        // Calculate widths to right-align the right section
        let left_len: usize = left_spans.iter().map(|s| s.content.len()).sum();
        let right_len: usize = right_spans.iter().map(|s| s.content.len()).sum();
        let padding = (area.width as usize).saturating_sub(left_len + right_len);

        // Combine with padding
        let mut all_spans = left_spans;
        all_spans.push(Span::raw(" ".repeat(padding)));
        all_spans.extend(right_spans);

        let status = Line::from(all_spans);
        let paragraph = Paragraph::new(status)
            .style(Style::default().bg(Color::Rgb(30, 30, 30)));

        frame.render_widget(paragraph, area);
    }

    /// Get the build/status indicator based on current session state
    fn get_build_status(&self) -> (&'static str, Style) {
        if let Some(session) = self.app.selected_session_data() {
            match &session.status {
                SessionStatus::Running { .. } => (
                    "Build",
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
                SessionStatus::Completed { .. } => (
                    "Done",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                SessionStatus::Error { .. } => (
                    "Error",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
                SessionStatus::Idle { .. } => (
                    "Idle",
                    Style::default().fg(Color::Gray),
                ),
            }
        } else {
            ("Ready", Style::default().fg(Color::Gray))
        }
    }

    /// Get contextual key hints based on focused panel
    fn get_key_hints(&self) -> Vec<(&'static str, &'static str)> {
        match self.app.focused_panel {
            FocusedPanel::Sessions => vec![
                ("↑↓", "navigate"),
                ("enter", "select"),
                ("h/l", "collapse/expand"),
                ("r", "add repo"),
                ("c-n", "new project"),
                ("esc", "exit"),
            ],
            FocusedPanel::Output => vec![
                ("↑↓", "scroll"),
                ("enter", "send"),
                ("p", "pause"),
                ("c-c", "interrupt"),
                ("?", "help"),
                ("esc", "exit"),
            ],
            FocusedPanel::Files => vec![
                ("↑↓", "navigate"),
                ("enter", "open"),
                ("h/l", "collapse/expand"),
                (".", "hidden"),
                ("e", "edit"),
                ("esc", "exit"),
            ],
        }
    }
}
