//! Status bar - displays context info and key hints
//!
//! A polished status bar showing build status, model info, current workspace,
//! and contextual keyboard shortcuts.

use crate::app::{App, FocusedPanel};
use crate::theme::{self, Theme};
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
        // Top line: Build status + Model info + Session name
        let top_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };

        // Bottom line: Key hints
        let bottom_area = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1.min(area.height.saturating_sub(1)),
        };

        self.render_top_line(frame, top_area);
        if area.height > 1 {
            self.render_bottom_line(frame, bottom_area);
        }
    }

    fn render_top_line(&self, frame: &mut Frame, area: Rect) {
        let mut left_spans: Vec<Span> = Vec::new();

        // Build status
        let (status_text, status_style) = self.get_build_status();
        left_spans.push(Span::styled(
            format!(" {} ", status_text),
            status_style,
        ));

        // Model info
        left_spans.push(Span::styled(
            " Opus 4.5 ",
            Style::default().fg(theme::accent::GREEN),
        ));
        left_spans.push(Span::styled(
            "Claude Code ",
            Style::default().fg(theme::text::SECONDARY),
        ));

        // Right side: session name
        let mut right_spans: Vec<Span> = Vec::new();
        if let Some(session) = self.app.selected_session_data() {
            right_spans.push(Span::styled(
                format!("{} ", session.name),
                Style::default().fg(theme::text::SECONDARY),
            ));
        }

        // Calculate padding
        let left_len: usize = left_spans.iter().map(|s| s.content.len()).sum();
        let right_len: usize = right_spans.iter().map(|s| s.content.len()).sum();
        let padding = (area.width as usize).saturating_sub(left_len + right_len);

        let mut all_spans = left_spans;
        all_spans.push(Span::raw(" ".repeat(padding)));
        all_spans.extend(right_spans);

        let line = Line::from(all_spans);
        let paragraph = Paragraph::new(line).style(Theme::status_bar_bg());
        frame.render_widget(paragraph, area);
    }

    fn render_bottom_line(&self, frame: &mut Frame, area: Rect) {
        let hints = self.get_key_hints();
        let mut spans: Vec<Span> = Vec::new();

        // Center the hints
        let total_len: usize = hints.iter()
            .map(|(k, v)| k.len() + 1 + v.len() + 2)
            .sum();
        let padding = (area.width as usize).saturating_sub(total_len) / 2;
        spans.push(Span::raw(" ".repeat(padding)));

        for (i, (key, action)) in hints.iter().enumerate() {
            spans.push(Span::styled(
                (*key).to_string(),
                Style::default().fg(theme::text::SECONDARY),
            ));
            spans.push(Span::styled(
                format!(" {}", action),
                Style::default().fg(theme::text::MUTED),
            ));
            if i < hints.len() - 1 {
                spans.push(Span::styled(
                    "   ",
                    Style::default(),
                ));
            }
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line).style(Theme::status_bar_bg());
        frame.render_widget(paragraph, area);
    }

    fn get_build_status(&self) -> (&'static str, Style) {
        if let Some(session) = self.app.selected_session_data() {
            match &session.status {
                SessionStatus::Running { .. } => (
                    "Build",
                    Style::default().fg(theme::accent::YELLOW),
                ),
                SessionStatus::Completed { .. } => (
                    "Done",
                    Style::default().fg(theme::accent::GREEN),
                ),
                SessionStatus::Error { .. } => (
                    "Error",
                    Style::default().fg(theme::accent::RED),
                ),
                SessionStatus::Idle { .. } => (
                    "Idle",
                    Style::default().fg(theme::text::MUTED),
                ),
            }
        } else {
            ("Ready", Style::default().fg(theme::text::MUTED))
        }
    }

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
