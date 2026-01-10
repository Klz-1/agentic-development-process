//! Output panel - displays live output from selected session
//!
//! Shows terminal output with a stats bar at the bottom displaying
//! session duration and data transfer metrics.

use crate::app::{App, FocusedPanel};
use crate::theme::{self, Theme};
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

    /// Check if we're viewing a session running hive itself (inception!)
    fn is_viewing_self(&self) -> bool {
        if let Some(session) = self.app.selected_session_data() {
            // Check if session name or project path contains "hive"
            let name_match = session.name.to_lowercase().contains("hive");
            let path_match = session
                .project_root
                .to_string_lossy()
                .to_lowercase()
                .contains("/hive");

            // Also check if output contains hive TUI markers
            let output_match = self.app.output_lines.iter().any(|line| {
                line.contains("Workspaces") && line.contains("Output") && line.contains("Files")
            });

            name_match || path_match || output_match
        } else {
            false
        }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let is_focused = self.app.focused_panel == FocusedPanel::Output;

        let session_name = self
            .app
            .selected_session_data()
            .map(|s| s.name.as_str())
            .unwrap_or("none");

        let title = format!(" {} ", session_name);

        let block = Block::default()
            .title(title)
            .title_style(Theme::title_style(is_focused))
            .borders(Borders::ALL)
            .border_style(Theme::border_style(is_focused))
            .style(Theme::panel_bg());

        let inner_area = block.inner(area);

        let chunks = Layout::vertical([
            Constraint::Min(1),    // Output content
            Constraint::Length(1), // Stats bar
            Constraint::Length(1), // Input line
        ])
        .split(inner_area);

        frame.render_widget(block, area);

        let total_lines = self.app.output_line_count;
        let visible_height = chunks[0].height as usize;

        // Check for inception (viewing hive itself)
        let is_inception = self.is_viewing_self();

        let output_lines: Vec<Line> = if is_inception {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  🐝 INCEPTION DETECTED 🐝",
                    Style::default().fg(theme::accent::YELLOW).bold(),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  You're trying to view hive... inside hive.",
                    Style::default().fg(theme::text::SECONDARY),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  We need to go deeper? No. No we don't.",
                    Style::default().fg(theme::text::MUTED).italic(),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  🎵 BWAAAAAM 🎵",
                    Style::default().fg(theme::accent::PEACH),
                )),
            ]
        } else if self.app.output_lines.is_empty() {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  No output yet...",
                    Style::default().fg(theme::text::MUTED),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  Select a session to view its output",
                    Style::default().fg(theme::text::MUTED),
                )),
            ]
        } else {
            self.app
                .output_lines
                .iter()
                .map(|line| Line::from(Span::styled(format!(" {}", line), Style::default().fg(theme::text::PRIMARY))))
                .collect()
        };

        let scroll_offset = self.app.output_scroll;
        let visible_lines: Vec<Line> = output_lines
            .into_iter()
            .skip(scroll_offset)
            .take(visible_height)
            .collect();

        let output_paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });
        frame.render_widget(output_paragraph, chunks[0]);

        self.render_stats_bar(frame, chunks[1]);

        // Input line
        let input_style = if is_focused && self.app.command_mode {
            Style::default().fg(theme::accent::YELLOW)
        } else if is_focused {
            Style::default().fg(theme::text::SECONDARY)
        } else {
            Style::default().fg(theme::text::MUTED)
        };

        let cursor = if is_focused { "_" } else { "" };
        let input_content = if self.app.command_mode {
            format!("{}{}", self.app.command_input, cursor)
        } else {
            cursor.to_string()
        };

        let input_line = Paragraph::new(Line::from(vec![
            Span::styled(" > ", input_style),
            Span::styled(input_content, Style::default().fg(theme::text::PRIMARY)),
        ]));

        frame.render_widget(input_line, chunks[2]);

        // Scrollbar
        if total_lines > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some(" "))
                .thumb_symbol("▐")
                .style(Theme::scrollbar_style())
                .thumb_style(Theme::scrollbar_thumb_style());

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

    fn render_stats_bar(&self, frame: &mut Frame, area: Rect) {
        let mut spans: Vec<Span> = Vec::new();

        spans.push(Span::styled(
            " ─ ",
            Style::default().fg(theme::border::DEFAULT),
        ));

        if let Some(session) = self.app.selected_session_data() {
            let duration = self.format_session_duration(session);
            spans.push(Span::styled(
                "⊙ ",
                Style::default().fg(theme::text::MUTED),
            ));
            spans.push(Span::styled(
                duration,
                Style::default().fg(theme::text::SECONDARY),
            ));

            spans.push(Span::styled(
                " │ ",
                Style::default().fg(theme::border::DEFAULT),
            ));

            spans.push(Span::styled(
                "↓",
                Style::default().fg(theme::accent::GREEN),
            ));
            spans.push(Span::styled(
                format!("{}k ", self.estimate_data_received()),
                Style::default().fg(theme::text::SECONDARY),
            ));

            spans.push(Span::styled(
                "↑",
                Style::default().fg(theme::accent::PEACH),
            ));
            spans.push(Span::styled(
                format!("{}k", self.estimate_data_sent()),
                Style::default().fg(theme::text::SECONDARY),
            ));

            if self.app.output_auto_scroll {
                spans.push(Span::styled(
                    " │ ",
                    Style::default().fg(theme::border::DEFAULT),
                ));
                spans.push(Span::styled(
                    "AUTO",
                    Style::default().fg(theme::text::MUTED),
                ));
            }
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, area);
    }

    fn format_session_duration(&self, session: &crate::tmux::Session) -> String {
        let duration = match &session.status {
            SessionStatus::Running { .. } => {
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

    fn estimate_data_received(&self) -> String {
        let bytes: usize = self.app.output_lines.iter().map(|s| s.len()).sum();
        Self::format_data_size(bytes)
    }

    fn estimate_data_sent(&self) -> String {
        let bytes = self.app.command_input.len() * 10;
        Self::format_data_size(bytes.max(100))
    }

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
