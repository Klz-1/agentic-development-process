//! UI components and rendering

mod files;
mod fuzzy;
mod help;
mod layout;
mod output;
mod sessions;
mod status_bar;
mod tab_bar;

use crate::app::App;
use crate::theme;
use ratatui::prelude::*;
use ratatui::widgets::Block;

pub use files::FilesPanel;
pub use fuzzy::FuzzyFinderOverlay;
pub use help::HelpOverlay;
pub use layout::create_layout_from_percentages;
pub use output::OutputPanel;
pub use sessions::SessionsPanel;
pub use status_bar::StatusBar;
pub use tab_bar::TabBar;

/// Draw the entire UI
pub fn draw(frame: &mut Frame, app: &App) {
    // Fill entire screen with dark background
    let bg_block = Block::default().style(Style::default().bg(theme::bg::DARK));
    frame.render_widget(bg_block, frame.area());

    let chunks = create_layout_from_percentages(frame.area(), app.panel_widths);

    // Draw tab bar at the top
    TabBar::new(app).render(frame, chunks.tab_bar);

    // Draw panels
    SessionsPanel::new(app).render(frame, chunks.sessions);
    OutputPanel::new(app).render(frame, chunks.output);
    FilesPanel::new(app).render(frame, chunks.files);
    StatusBar::new(app).render(frame, chunks.status_bar);

    // Draw overlays on top
    if app.show_help {
        HelpOverlay::render(frame);
    }

    if app.show_fuzzy_finder {
        FuzzyFinderOverlay::new(&app.fuzzy_input).render(frame);
    }
}
