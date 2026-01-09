//! Output panel - displays live output from selected session

use crate::app::{App, FocusedPanel};
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

        // Build title with session name and scroll indicator
        let total_lines = self.app.output_line_count;
        let visible_height = area.height.saturating_sub(4) as usize;

        let session_name = self
            .app
            .selected_session_data()
            .map(|s| s.name.as_str())
            .unwrap_or("none");

        let title = format!(" Output: {} ", session_name);

        let scroll_status = if self.app.output_auto_scroll {
            " [AUTO] ".to_string()
        } else if total_lines > visible_height {
            format!(
                " [{}..{}] ",
                self.app.output_scroll + 1,
                (self.app.output_scroll + visible_height).min(total_lines)
            )
        } else {
            String::new()
        };

        let block = Block::default()
            .title(title)
            .title_bottom(Line::from(scroll_status).right_aligned())
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner_area = block.inner(area);

        // Split inner area: output content + input line
        let chunks = Layout::vertical([
            Constraint::Min(1),    // Output content
            Constraint::Length(1), // Input line
        ])
        .split(inner_area);

        // Render the block first
        frame.render_widget(block, area);

        // Convert output lines to ratatui Lines
        let output_lines: Vec<Line> = if self.app.output_lines.is_empty() {
            vec![
                Line::from(Span::styled(
                    "No output yet...",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Select a session to view its output",
                    Style::default().fg(Color::DarkGray),
                )),
            ]
        } else {
            self.app
                .output_lines
                .iter()
                .map(|line| Line::from(line.as_str()))
                .collect()
        };

        // Apply scroll offset
        let scroll_offset = self.app.output_scroll;
        let visible_lines: Vec<Line> = output_lines
            .into_iter()
            .skip(scroll_offset)
            .take(chunks[0].height as usize)
            .collect();

        let output_paragraph = Paragraph::new(visible_lines).wrap(Wrap { trim: false });

        frame.render_widget(output_paragraph, chunks[0]);

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
            Span::styled("> ", input_style),
            Span::styled(
                input_content,
                Style::default().add_modifier(Modifier::SLOW_BLINK),
            ),
        ]));

        frame.render_widget(input_line, chunks[1]);

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
}
