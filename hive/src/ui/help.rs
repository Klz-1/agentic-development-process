//! Help overlay - displays keyboard shortcuts

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

pub struct HelpOverlay;

impl HelpOverlay {
    pub fn render(frame: &mut Frame) {
        let area = centered_rect(60, 70, frame.area());

        // Clear the background
        frame.render_widget(Clear, area);

        let help_text = vec![
            Line::from(""),
            Line::from(Span::styled("  Navigation", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  Tab        Cycle panel focus"),
            Line::from("  ↑/↓        Navigate list"),
            Line::from("  Enter      Primary action"),
            Line::from("  Space      Secondary action"),
            Line::from(""),
            Line::from(Span::styled("  Sessions Panel", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  Enter      Attach to session"),
            Line::from("  d          Send interrupt (Ctrl+C)"),
            Line::from(""),
            Line::from(Span::styled("  Output Panel", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  Enter      Send command"),
            Line::from("  p          Pause/resume scroll"),
            Line::from(""),
            Line::from(Span::styled("  Files Panel", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  Enter      Preview/open file"),
            Line::from("  e          Edit in $EDITOR"),
            Line::from("  Tab        Expand/collapse directory"),
            Line::from(""),
            Line::from(Span::styled("  Global", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from("  /          Fuzzy finder"),
            Line::from("  ?          Toggle help"),
            Line::from("  q          Quit"),
            Line::from("  Esc        Cancel/back"),
            Line::from(""),
        ];

        let block = Block::default()
            .title(" Help ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let paragraph = Paragraph::new(help_text).block(block);

        frame.render_widget(paragraph, area);
    }
}

/// Create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
