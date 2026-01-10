//! Tab bar component for workspace switching
//!
//! Displays tabs for each active workspace/session at the top of the UI,
//! inspired by IDE-style tab navigation.

use crate::app::App;
use crate::theme;
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
            spans.push(Span::styled(
                " No active sessions ",
                Style::default().fg(theme::text::MUTED),
            ));
            spans.push(Span::styled(
                "[+] New",
                Style::default().fg(theme::accent::CYAN),
            ));
        } else {
            for (i, session) in self.app.sessions.iter().enumerate() {
                let is_selected = i == self.app.selected_session;

                // Tab number
                let num_style = if is_selected {
                    Style::default().fg(theme::text::SECONDARY)
                } else {
                    Style::default().fg(theme::text::MUTED)
                };
                spans.push(Span::styled(format!("[{}] ", i + 1), num_style));

                let (project, workspace) = Self::parse_session_name(&session.name);

                // Tab content
                let tab_style = if is_selected {
                    Style::default()
                        .fg(theme::text::PRIMARY)
                        .bg(theme::bg::HIGHLIGHT)
                } else {
                    Style::default().fg(theme::text::SECONDARY)
                };

                let tab_text = if let Some(proj) = project {
                    format!("{} ({})", proj, workspace)
                } else {
                    workspace.to_string()
                };

                spans.push(Span::styled(format!(" {} ", tab_text), tab_style));

                if i < self.app.sessions.len() - 1 {
                    spans.push(Span::styled(" ", Style::default()));
                }
            }

            // Add "new tab" button
            spans.push(Span::styled("  ", Style::default()));
            spans.push(Span::styled(
                "[+] New",
                Style::default().fg(theme::accent::CYAN),
            ));
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line)
            .style(Style::default().bg(theme::bg::DARK));

        frame.render_widget(paragraph, area);
    }

    fn parse_session_name(name: &str) -> (Option<&str>, &str) {
        if let Some((project, workspace)) = name.split_once('/') {
            return (Some(project), workspace);
        }
        if let Some((project, workspace)) = name.rsplit_once('-') {
            if !project.is_empty() && !workspace.is_empty() && project.len() > 2 {
                return (Some(project), workspace);
            }
        }
        (None, name)
    }
}
