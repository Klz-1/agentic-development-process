//! Output panel - displays live output from selected session
//!
//! Shows terminal output with a stats bar at the bottom displaying
//! session duration and data transfer metrics.

use crate::app::{App, FocusedPanel};
use crate::tmux::SessionStatus;
use chrono::Utc;
use ratatui::prelude::*;
use ratatui::widgets::{
    Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};

pub struct OutputPanel<'a> {
    app: &'a App,
}

impl<'a> OutputPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let is_focused = self.app.focused_panel == FocusedPanel::Output;

        let border_style = if is_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        // Build title with session name
        let session_name = self
            .app
            .selected_session_data()
            .map(|s| s.name.as_str())
            .unwrap_or("none");

        let title = format!(" {} ", session_name);

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner_area = block.inner(area);

        // Split inner area: output content + stats bar + input line
        let chunks = Layout::vertical([
            Constraint::Min(1),    // Output content
            Constraint::Length(1), // Stats bar
            Constraint::Length(1), // Input line
        ])
        .split(inner_area);

        // Render the block first
        frame.render_widget(block, area);

        // Calculate visible height for scrolling
        let total_lines = self.app.output_line_count;
        let visible_height = chunks[0].height as usize;

        // Convert output lines to ratatui Lines
        let output_lines: Vec<Line> = if self.app.output_lines.is_empty() {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  No output yet...",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Select a session to view its output",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        } else {
            self.app
                .output_lines
                .iter()
                .map(|line| Line::from(format!(" {}", line)))
                .collect()
        };

        // Apply scroll offset
        let scroll_offset = self.app.output_scroll;
        let visible_lines: Vec<Line> = output_lines
            .into_iter()
            .skip(scroll_offset)
            .take(visible_height)
            .collect();

        let output_paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });
        frame.render_widget(output_paragraph, chunks[0]);

        // Render stats bar
        self.render_stats_bar(frame, chunks[1]);

        // Render input line at bottom
        let input_style = if is_focused && self.app.command_mode {
            Style::default().fg(Color::Yellow)
        } else if is_focused {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let cursor = if is_focused { "_" } else { "" };
        let input_content = if self.app.command_mode {
            format!("{}{}", self.app.command_input, cursor)
        } else {
            cursor.to_string()
        };

        let input_line = Paragraph::new(Line::from(vec![
            Span::styled(" > ", input_style),
            Span::styled(
                input_content,
                Style::default().add_modifier(Modifier::SLOW_BLINK),
            ),
        ]));

        frame.render_widget(input_line, chunks[2]);

        // Render scrollbar if content exceeds visible area
        if total_lines > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"))
                .track_symbol(Some("│"))
                .thumb_symbol("█");

            let mut scrollbar_state =
                ScrollbarState::new(total_lines).position(self.app.output_scroll);

            let scrollbar_area = Rect {
                x: area.x + area.width - 2,
                y: area.y + 1,
                width: 1,
                height: chunks[0].height,
            };

            frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }

    /// Render the stats bar showing session duration and metrics
    fn render_stats_bar(&self, frame: &mut Frame, area: Rect) {
        let mut spans: Vec<Span> = Vec::new();

        // Separator line character
        spans.push(Span::styled(
            " ─ ",
            Style::default().fg(Color::DarkGray),
        ));

        // Session duration
        if let Some(session) = self.app.selected_session_data() {
            let duration = self.format_session_duration(session);
            spans.push(Span::styled(
                "⊙ ",
                Style::default().fg(Color::DarkGray),
            ));
            spans.push(Span::styled(
                duration,
                Style::default().fg(Color::Gray),
            ));

            // Data transfer stats (simulated - would come from real metrics)
            spans.push(Span::styled(
                " │ ",
                Style::default().fg(Color::DarkGray),
            ));

            // Download indicator
            spans.push(Span::styled(
                "↓",
                Style::default().fg(Color::Green),
            ));
            spans.push(Span::styled(
                format!("{}k ", self.estimate_data_received()),
                Style::default().fg(Color::Gray),
            ));

            // Upload indicator
            spans.push(Span::styled(
                "↑",
                Style::default().fg(Color::Yellow),
            ));
            spans.push(Span::styled(
                format!("{}k", self.estimate_data_sent()),
                Style::default().fg(Color::Gray),
            ));

            // Scroll indicator
            if self.app.output_auto_scroll {
                spans.push(Span::styled(
                    " │ ",
                    Style::default().fg(Color::DarkGray),
                ));
                spans.push(Span::styled(
                    "AUTO",
                    Style::default().fg(Color::Cyan),
                ));
            }
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, area);
    }

    /// Format session duration as human-readable string
    fn format_session_duration(&self, session: &crate::tmux::Session) -> String {
        let duration = match &session.status {
            SessionStatus::Running { .. } => {
                // Calculate from session start (using last_activity as proxy)
                Utc::now().signed_duration_since(session.last_activity)
            }
            SessionStatus::Idle { since } => {
                Utc::now().signed_duration_since(*since)
            }
            SessionStatus::Completed { at } => {
                Utc::now().signed_duration_since(*at)
            }
            SessionStatus::Error { .. } => {
                Utc::now().signed_duration_since(session.last_activity)
            }
        };

        let total_seconds = duration.num_seconds().max(0);
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    /// Estimate data received (based on output buffer size)
    fn estimate_data_received(&self) -> String {
        let bytes: usize = self.app.output_lines.iter().map(|s| s.len()).sum();
        Self::format_data_size(bytes)
    }

    /// Estimate data sent (based on commands sent - simplified)
    fn estimate_data_sent(&self) -> String {
        // Simplified: estimate based on command input length
        let bytes = self.app.command_input.len() * 10; // rough estimate
        Self::format_data_size(bytes.max(100)) // minimum 100 bytes
    }

    /// Format byte size as human-readable string
    fn format_data_size(bytes: usize) -> String {
        if bytes >= 1_000_000 {
            format!("{:.1}M", bytes as f64 / 1_000_000.0)
        } else if bytes >= 1_000 {
            format!("{:.1}", bytes as f64 / 1_000.0)
        } else {
            format!("0.{}", bytes / 100)
        }
    }
}
