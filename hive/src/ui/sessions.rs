//! Sessions panel - displays workspaces in a simple list view

use crate::app::{App, FocusedPanel};
use crate::theme::{self, Theme};
use crate::tmux::{Session, SessionStatus};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState};

pub struct SessionsPanel<'a> {
    app: &'a App,
}

impl<'a> SessionsPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    /// Get status icon for a session
    fn status_icon(session: &Session) -> (&'static str, Style) {
        match &session.status {
            SessionStatus::Running { .. } => ("●", Style::default().fg(theme::accent::GREEN)),
            SessionStatus::Idle { .. } => ("○", Style::default().fg(theme::text::MUTED)),
            SessionStatus::Error { .. } => ("●", Style::default().fg(theme::accent::RED)),
            SessionStatus::Completed { .. } => ("●", Style::default().fg(theme::accent::GREEN)),
        }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let is_focused = self.app.focused_panel == FocusedPanel::Sessions;

        let block = Block::default()
            .title(" Workspaces ")
            .title_style(Theme::title_style(is_focused))
            .borders(Borders::ALL)
            .border_style(Theme::border_style(is_focused))
            .style(Theme::panel_bg());

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Build simple list - one session per row
        let items: Vec<ListItem> = if self.app.sessions.is_empty() {
            vec![
                ListItem::new(Line::from(Span::styled(
                    "  No workspaces",
                    Style::default().fg(theme::text::MUTED),
                ))),
                ListItem::new(Line::from(Span::styled(
                    "  + New workspace",
                    Theme::link_style(),
                ))),
            ]
        } else {
            self.app
                .sessions
                .iter()
                .map(|session| {
                    let (icon, icon_style) = Self::status_icon(session);

                    ListItem::new(Line::from(vec![
                        Span::styled(" ", Style::default()),
                        Span::styled(icon, icon_style),
                        Span::styled(" ", Style::default()),
                        Span::styled(&session.name, Style::default().fg(theme::text::PRIMARY)),
                    ]))
                })
                .collect()
        };

        let list = List::new(items)
            .highlight_style(Theme::highlight_style())
            .highlight_symbol("▸ ");

        let mut state = ListState::default();
        if !self.app.sessions.is_empty() {
            state.select(Some(self.app.selected_session));
        }

        frame.render_stateful_widget(list, inner_area, &mut state);

        // Render scrollbar if needed
        let total_items = self.app.sessions.len().max(2); // At least 2 for empty state
        let visible_height = inner_area.height as usize;
        if total_items > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some(" "))
                .thumb_symbol("▐")
                .style(Theme::scrollbar_style())
                .thumb_style(Theme::scrollbar_thumb_style());

            let mut scrollbar_state =
                ScrollbarState::new(total_items).position(self.app.sessions_scroll);

            let scrollbar_area = Rect {
                x: inner_area.x + inner_area.width,
                y: inner_area.y,
                width: 1,
                height: inner_area.height,
            };

            frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }
}
