//! Status bar - displays context info and key hints

use crate::app::{App, FocusedPanel};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct StatusBar<'a> {
    app: &'a App,
}

impl<'a> StatusBar<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let panel_hint = match self.app.focused_panel {
            FocusedPanel::Sessions => "Enter: attach │ d: interrupt",
            FocusedPanel::Output => "Enter: send │ p: pause",
            FocusedPanel::Files => "Enter: open │ e: edit",
        };

        let status = Line::from(vec![
            Span::styled(" 3 active ", Style::default().fg(Color::Green)),
            Span::raw("│"),
            Span::styled(" 1 error ", Style::default().fg(Color::Red)),
            Span::raw("│"),
            Span::styled(" 1 idle ", Style::default().fg(Color::DarkGray)),
            Span::raw(" │ "),
            Span::styled("webapp-agent", Style::default().fg(Color::Cyan)),
            Span::raw(" │ "),
            Span::styled(panel_hint, Style::default().fg(Color::DarkGray)),
            Span::raw(" │ "),
            Span::styled("?: help  q: quit", Style::default().fg(Color::DarkGray)),
        ]);

        let paragraph = Paragraph::new(status)
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));

        frame.render_widget(paragraph, area);
    }
}
