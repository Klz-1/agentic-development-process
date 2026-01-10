//! Sessions panel - displays workspaces in a hierarchical tree view
//!
//! Styled as a "Workspaces" sidebar with collapsible project groups,
//! similar to IDE-style project navigation.

use crate::app::{App, FocusedPanel};
use crate::tmux::{Session, SessionStatus};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState};
use std::collections::BTreeMap;

pub struct SessionsPanel<'a> {
    app: &'a App,
}

impl<'a> SessionsPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    /// Parse session name to extract project and workspace
    /// Handles formats: "project/workspace", "project-workspace", "workspace"
    fn parse_session_name(name: &str) -> (String, String) {
        if let Some((project, workspace)) = name.split_once('/') {
            return (project.to_string(), workspace.to_string());
        }
        // For names like "project-workspace", use the first part as project
        if let Some((project, workspace)) = name.split_once('-') {
            if !project.is_empty() && !workspace.is_empty() && project.len() > 2 {
                return (project.to_string(), workspace.to_string());
            }
        }
        // Default: put in "default" project
        ("default".to_string(), name.to_string())
    }

    /// Group session indices by project name
    fn group_sessions_by_project(sessions: &[Session]) -> BTreeMap<String, Vec<usize>> {
        let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();

        for (idx, session) in sessions.iter().enumerate() {
            let (project, _workspace) = Self::parse_session_name(&session.name);
            groups.entry(project).or_default().push(idx);
        }

        groups
    }

    /// Get status icon for a session
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

        let block = Block::default()
            .title(" Workspaces ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        // Build list items
        let mut items: Vec<ListItem> = Vec::new();
        let mut session_index_map: Vec<Option<usize>> = Vec::new(); // Maps list index to session index

        if self.app.sessions.is_empty() {
            // Empty state
            items.push(ListItem::new(Line::from(vec![
                Span::styled("  No workspaces", Style::default().fg(Color::DarkGray)),
            ])));
            session_index_map.push(None);

            items.push(ListItem::new(Line::from(vec![
                Span::styled("  ", Style::default()),
            ])));
            session_index_map.push(None);

            items.push(ListItem::new(Line::from(vec![
                Span::styled("+ ", Style::default().fg(Color::Cyan)),
                Span::styled("New workspace", Style::default().fg(Color::Cyan)),
            ])));
            session_index_map.push(None);
        } else {
            let groups = Self::group_sessions_by_project(&self.app.sessions);

            for (project_name, session_indices) in groups {
                // Project header with collapse indicator
                let collapse_icon = "▼"; // All expanded by default
                items.push(ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("{} ", collapse_icon),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::styled(
                        project_name.clone(),
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                    ),
                ])));
                session_index_map.push(None); // Header, not a session

                // "+ New workspace" button
                items.push(ListItem::new(Line::from(vec![
                    Span::styled("  + ", Style::default().fg(Color::Cyan)),
                    Span::styled("New workspace", Style::default().fg(Color::DarkGray)),
                ])));
                session_index_map.push(None);

                // Workspaces in this project
                for session_idx in session_indices {
                    let session = &self.app.sessions[session_idx];
                    let (_project, workspace) = Self::parse_session_name(&session.name);
                    let (icon, icon_style) = Self::status_icon(session);
                    let is_selected = session_idx == self.app.selected_session;

                    let name_style = if is_selected && is_focused {
                        Style::default()
                            .fg(Color::White)
                            .bg(Color::Rgb(50, 50, 50))
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    };

                    // Show full session name or just workspace part
                    let display_name = format!("{}/{}", _project, workspace);

                    items.push(ListItem::new(Line::from(vec![
                        Span::styled("  ", Style::default()),
                        Span::styled(icon, icon_style),
                        Span::styled(" ", Style::default()),
                        Span::styled(display_name, name_style),
                    ])));
                    session_index_map.push(Some(session_idx));

                    // Secondary line with just the workspace name (dimmed)
                    items.push(ListItem::new(Line::from(vec![
                        Span::styled("    ", Style::default()),
                        Span::styled(
                            workspace,
                            Style::default().fg(Color::DarkGray),
                        ),
                    ])));
                    session_index_map.push(Some(session_idx));
                }
            }
        }

        // Find which list item to highlight based on selected session
        let highlight_index = session_index_map
            .iter()
            .position(|&idx| idx == Some(self.app.selected_session));

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(Color::Rgb(40, 40, 40)),
            )
            .highlight_symbol("");

        let mut state = ListState::default();
        state.select(highlight_index);

        frame.render_stateful_widget(list, inner_area, &mut state);

        // Render scrollbar if needed
        let total_items = session_index_map.len();
        let visible_height = inner_area.height as usize;
        if total_items > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"))
                .track_symbol(Some("│"))
                .thumb_symbol("█");

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
