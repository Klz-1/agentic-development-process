//! Output panel - displays live output from selected session

use crate::app::{App, FocusedPanel};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};

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

        // Build title with scroll indicator
        let total_lines = self.app.output_line_count;
        let visible_height = area.height.saturating_sub(4) as usize; // Account for borders and input line

        let scroll_status = if self.app.output_auto_scroll {
            " [AUTO] ".to_string()
        } else if total_lines > visible_height {
            format!(" [{}..{}] ", self.app.output_scroll + 1, (self.app.output_scroll + visible_height).min(total_lines))
        } else {
            String::new()
        };

        let block = Block::default()
            .title(" Output ")
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

        // Mock output data with timestamps
        let output_lines = vec![
            Line::from(vec![
                Span::styled("[15:42:03] ", Style::default().fg(Color::DarkGray)),
                Span::raw("Running tests..."),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("[15:42:05] ", Style::default().fg(Color::DarkGray)),
                Span::styled("✓ ", Style::default().fg(Color::Green)),
                Span::raw("42/42 tests passed"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("[15:42:06] ", Style::default().fg(Color::DarkGray)),
                Span::raw("Building release..."),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("[15:42:10] ", Style::default().fg(Color::DarkGray)),
                Span::styled("✓ ", Style::default().fg(Color::Green)),
                Span::raw("Completed successfully"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("[15:42:11] ", Style::default().fg(Color::DarkGray)),
                Span::raw("Deploying to staging..."),
            ]),
        ];

        // Apply scroll offset
        let scroll_offset = self.app.output_scroll;
        let visible_lines: Vec<Line> = output_lines
            .into_iter()
            .skip(scroll_offset)
            .take(chunks[0].height as usize)
            .collect();

        let output_paragraph = Paragraph::new(visible_lines)
            .wrap(Wrap { trim: false });

        frame.render_widget(output_paragraph, chunks[0]);

        // Render input line at bottom
        let input_style = if is_focused {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let cursor = if is_focused { "_" } else { "" };
        let input_line = Paragraph::new(Line::from(vec![
            Span::styled("> ", input_style),
            Span::styled(cursor, Style::default().add_modifier(Modifier::SLOW_BLINK)),
        ]));

        frame.render_widget(input_line, chunks[1]);

        // Render scrollbar if content exceeds visible area
        if total_lines > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"))
                .track_symbol(Some("│"))
                .thumb_symbol("█");

            let mut scrollbar_state = ScrollbarState::new(total_lines)
                .position(self.app.output_scroll);

            // Render scrollbar in the output content area
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
