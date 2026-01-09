//! Sessions panel - displays tmux sessions with status

use crate::app::{App, FocusedPanel};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState};

/// Mock session data for demonstration
struct MockSession {
    name: &'static str,
    status: SessionDisplayStatus,
    detail: &'static str,
}

enum SessionDisplayStatus {
    Running,
    Idle,
    Error,
}

impl MockSession {
    fn to_list_items(&self) -> Vec<ListItem<'static>> {
        let (icon, style) = match self.status {
            SessionDisplayStatus::Running => ("●", Style::default().fg(Color::Green)),
            SessionDisplayStatus::Idle => ("○", Style::default().fg(Color::DarkGray)),
            SessionDisplayStatus::Error => ("⚠", Style::default().fg(Color::Red)),
        };

        vec![
            ListItem::new(Line::from(vec![
                Span::styled(icon, style),
                Span::raw(" "),
                Span::raw(self.name),
            ])),
            ListItem::new(Line::from(vec![
                Span::raw("  "),
                Span::styled(self.detail, Style::default().fg(Color::DarkGray)),
            ])),
        ]
    }
}

pub struct SessionsPanel<'a> {
    app: &'a App,
}

impl<'a> SessionsPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let is_focused = self.app.focused_panel == FocusedPanel::Sessions;

        let border_style = if is_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        // Calculate scroll indicator
        let total_items = self.app.session_count;
        let visible_height = area.height.saturating_sub(2) as usize; // Account for borders
        let scroll_indicator = if total_items > visible_height {
            format!(" [{}/{}] ", self.app.selected_session + 1, total_items)
        } else {
            String::new()
        };

        let block = Block::default()
            .title(" Sessions ")
            .title_bottom(Line::from(scroll_indicator).right_aligned())
            .borders(Borders::ALL)
            .border_style(border_style);

        // Mock session data
        let sessions = [
            MockSession { name: "webapp-agent", status: SessionDisplayStatus::Running, detail: "CPU: 12% MEM: 2%" },
            MockSession { name: "api-refactor", status: SessionDisplayStatus::Idle, detail: "Idle 5m" },
            MockSession { name: "data-migration", status: SessionDisplayStatus::Error, detail: "Error - check" },
            MockSession { name: "test-runner", status: SessionDisplayStatus::Running, detail: "Running tests..." },
        ];

        // Build list items with session groups
        let mut items: Vec<ListItem> = Vec::new();
        for (i, session) in sessions.iter().enumerate() {
            let session_items = session.to_list_items();
            for item in session_items {
                items.push(item);
            }
            // Add spacing between sessions (except after last)
            if i < sessions.len() - 1 {
                items.push(ListItem::new(""));
            }
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            )
            .highlight_symbol("▶ ");

        let mut state = ListState::default();
        state.select(Some(self.app.selected_session));

        frame.render_stateful_widget(list, area, &mut state);

        // Render scrollbar if content exceeds visible area
        if total_items > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"))
                .track_symbol(Some("│"))
                .thumb_symbol("█");

            let mut scrollbar_state = ScrollbarState::new(total_items)
                .position(self.app.sessions_scroll);

            // Render scrollbar in the area inside the block
            let scrollbar_area = Rect {
                x: area.x + area.width - 1,
                y: area.y + 1,
                width: 1,
                height: area.height.saturating_sub(2),
            };

            frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }
}
