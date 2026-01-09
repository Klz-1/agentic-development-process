//! Sessions panel - displays tmux sessions with status

use crate::app::{App, FocusedPanel};
use crate::tmux::{Session, SessionStatus};
use chrono::Utc;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState};

pub struct SessionsPanel<'a> {
    app: &'a App,
}

impl<'a> SessionsPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    /// Format session status as a display string
    fn format_status(session: &Session) -> (String, Style) {
        match &session.status {
            SessionStatus::Running { progress } => {
                let detail = if let Some(p) = progress {
                    format!(
                        "{:.0}% {}",
                        p.percentage() * 100.0,
                        p.label.as_deref().unwrap_or("")
                    )
                } else {
                    format!(
                        "CPU: {:.0}% MEM: {}MB",
                        session.resource_usage.cpu_percent,
                        session.resource_usage.memory_mb
                    )
                };
                (detail, Style::default().fg(Color::Green))
            }
            SessionStatus::Idle { since } => {
                let duration = Utc::now().signed_duration_since(*since);
                let idle_str = if duration.num_hours() > 0 {
                    format!("Idle {}h", duration.num_hours())
                } else if duration.num_minutes() > 0 {
                    format!("Idle {}m", duration.num_minutes())
                } else {
                    "Idle".to_string()
                };
                (idle_str, Style::default().fg(Color::DarkGray))
            }
            SessionStatus::Error { message } => {
                let short_msg = if message.len() > 20 {
                    format!("{}...", &message[..17])
                } else {
                    message.clone()
                };
                (short_msg, Style::default().fg(Color::Red))
            }
            SessionStatus::Completed { at } => {
                let duration = Utc::now().signed_duration_since(*at);
                let completed_str = if duration.num_hours() > 0 {
                    format!("Done {}h ago", duration.num_hours())
                } else if duration.num_minutes() > 0 {
                    format!("Done {}m ago", duration.num_minutes())
                } else {
                    "Just completed".to_string()
                };
                (completed_str, Style::default().fg(Color::Blue))
            }
        }
    }

    /// Get status icon and color
    fn status_icon(session: &Session) -> (&'static str, Style) {
        match &session.status {
            SessionStatus::Running { .. } => ("●", Style::default().fg(Color::Green)),
            SessionStatus::Idle { .. } => ("○", Style::default().fg(Color::DarkGray)),
            SessionStatus::Error { .. } => ("⚠", Style::default().fg(Color::Red)),
            SessionStatus::Completed { .. } => ("✓", Style::default().fg(Color::Blue)),
        }
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
        let visible_height = area.height.saturating_sub(2) as usize;
        let scroll_indicator = if total_items > 0 {
            format!(" [{}/{}] ", self.app.selected_session + 1, total_items)
        } else {
            " [no sessions] ".to_string()
        };

        let block = Block::default()
            .title(" Sessions ")
            .title_bottom(Line::from(scroll_indicator).right_aligned())
            .borders(Borders::ALL)
            .border_style(border_style);

        // Build list items from real session data
        let mut items: Vec<ListItem> = Vec::new();

        if self.app.sessions.is_empty() {
            // Show placeholder when no sessions
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  No tmux sessions", Style::default().fg(Color::DarkGray)),
            ])));
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  Run 'tmux new -s name'", Style::default().fg(Color::DarkGray)),
            ])));
        } else {
            for (i, session) in self.app.sessions.iter().enumerate() {
                let (icon, icon_style) = Self::status_icon(session);
                let (detail, _detail_style) = Self::format_status(session);

                // Highlight selected session
                let name_style = if i == self.app.selected_session && is_focused {
                    Style::default().add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                items.push(ListItem::new(Line::from(vec![
                    Span::styled(icon, icon_style),
                    Span::raw(" "),
                    Span::styled(&session.name, name_style),
                ])));

                items.push(ListItem::new(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(detail, Style::default().fg(Color::DarkGray)),
                ])));

                // Add spacing between sessions (except after last)
                if i < self.app.sessions.len() - 1 {
                    items.push(ListItem::new(""));
                }
            }
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray),
            )
            .highlight_symbol("▶ ");

        // Map selected session to list index (each session takes 2-3 list items)
        let list_index = if self.app.sessions.is_empty() {
            None
        } else {
            Some(self.app.selected_session * 3) // 2 lines + 1 spacer per session
        };

        let mut state = ListState::default();
        state.select(list_index);

        frame.render_stateful_widget(list, area, &mut state);

        // Render scrollbar if content exceeds visible area
        if total_items > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"))
                .track_symbol(Some("│"))
                .thumb_symbol("█");

            let mut scrollbar_state =
                ScrollbarState::new(total_items).position(self.app.sessions_scroll);

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
