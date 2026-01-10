//! Tab bar component for workspace switching
//!
//! Displays tabs for each active workspace/session at the top of the UI,
//! inspired by IDE-style tab navigation.

use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct TabBar<'a> {
    app: &'a App,
}

impl<'a> TabBar<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let mut spans: Vec<Span> = Vec::new();

        if self.app.sessions.is_empty() {
            // Show placeholder when no sessions
            spans.push(Span::styled(
                " No active sessions ",
                Style::default().fg(Color::DarkGray),
            ));
            spans.push(Span::styled(
                "[+] New",
                Style::default().fg(Color::Cyan),
            ));
        } else {
            // Build tabs for each session
            for (i, session) in self.app.sessions.iter().enumerate() {
                let is_selected = i == self.app.selected_session;

                // Tab number
                let num_style = if is_selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                spans.push(Span::styled(format!("[{}] ", i + 1), num_style));

                // Extract project and workspace name from session name
                // Assume format: "project/workspace" or just "workspace"
                let (project, workspace) = Self::parse_session_name(&session.name);

                // Tab content
                let tab_style = if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };

                let tab_text = if let Some(proj) = project {
                    format!("{} ({})", proj, workspace)
                } else {
                    workspace.to_string()
                };

                spans.push(Span::styled(format!(" {} ", tab_text), tab_style));

                // Separator between tabs
                if i < self.app.sessions.len() - 1 {
                    spans.push(Span::styled(" ", Style::default()));
                }
            }

            // Add "new tab" button at the end
            spans.push(Span::styled("  ", Style::default()));
            spans.push(Span::styled(
                "[+] New",
                Style::default().fg(Color::Cyan),
            ));
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line)
            .style(Style::default().bg(Color::Black));

        frame.render_widget(paragraph, area);
    }

    /// Parse session name into (project, workspace) tuple
    /// Handles formats like "project/workspace", "project-workspace", or just "workspace"
    fn parse_session_name(name: &str) -> (Option<&str>, &str) {
        // Try splitting by common separators
        if let Some((project, workspace)) = name.split_once('/') {
            return (Some(project), workspace);
        }
        if let Some((project, workspace)) = name.rsplit_once('-') {
            // Only use hyphen if it looks like project-workspace (not just-a-name)
            if !project.is_empty() && !workspace.is_empty() && project.len() > 2 {
                return (Some(project), workspace);
            }
        }
        (None, name)
    }
}
