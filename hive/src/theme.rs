//! Theme module - Clean dark theme inspired by modern IDEs
//!
//! Provides a cohesive color palette for the Hive TUI.
//! Dark background with subtle borders and minimal bright accents.

use ratatui::prelude::*;

/// Base background colors
pub mod bg {
    use super::*;

    /// Main background - very dark
    pub const DARK: Color = Color::Rgb(26, 27, 38);

    /// Panel background - slightly lighter
    pub const PANEL: Color = Color::Rgb(30, 32, 44);

    /// Selection/highlight background
    pub const HIGHLIGHT: Color = Color::Rgb(45, 48, 65);

    /// Status bar background
    pub const STATUS: Color = Color::Rgb(35, 38, 52);
}

/// Text colors
pub mod text {
    use super::*;

    /// Primary text - bright white for readability
    pub const PRIMARY: Color = Color::Rgb(205, 214, 244);

    /// Secondary text - muted gray
    pub const SECONDARY: Color = Color::Rgb(147, 153, 178);

    /// Muted/dim text
    pub const MUTED: Color = Color::Rgb(88, 91, 112);
}

/// Accent colors - used sparingly
pub mod accent {
    use super::*;

    /// Cyan - for links and interactive elements
    pub const CYAN: Color = Color::Rgb(137, 180, 250);

    /// Green - success/running
    pub const GREEN: Color = Color::Rgb(166, 218, 149);

    /// Yellow - warnings/build status
    pub const YELLOW: Color = Color::Rgb(249, 226, 175);

    /// Red - errors
    pub const RED: Color = Color::Rgb(243, 139, 168);

    /// Peach - for special highlights
    pub const PEACH: Color = Color::Rgb(250, 179, 135);
}

/// Border colors - subtle, not distracting
pub mod border {
    use super::*;

    /// Default border - very subtle
    pub const DEFAULT: Color = Color::Rgb(49, 50, 68);

    /// Focused border - slightly brighter but still subtle
    pub const FOCUS: Color = Color::Rgb(88, 91, 112);
}

/// Theme struct for easy access to styling methods
#[derive(Debug, Clone, Copy)]
pub struct Theme;

impl Theme {
    /// Get border style based on focus state
    pub fn border_style(focused: bool) -> Style {
        if focused {
            Style::default().fg(border::FOCUS)
        } else {
            Style::default().fg(border::DEFAULT)
        }
    }

    /// Get title style based on focus state
    pub fn title_style(focused: bool) -> Style {
        if focused {
            Style::default().fg(text::PRIMARY).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(text::MUTED)
        }
    }

    /// Get highlight style for selected items
    pub fn highlight_style() -> Style {
        Style::default()
            .bg(bg::HIGHLIGHT)
            .fg(text::PRIMARY)
    }

    /// Get panel background style
    pub fn panel_bg() -> Style {
        Style::default().bg(bg::PANEL)
    }

    /// Get scrollbar style
    pub fn scrollbar_style() -> Style {
        Style::default().fg(border::DEFAULT)
    }

    /// Get scrollbar thumb style
    pub fn scrollbar_thumb_style() -> Style {
        Style::default().fg(text::MUTED)
    }

    /// Get style for status running/build
    pub fn status_running() -> Style {
        Style::default().fg(accent::YELLOW)
    }

    /// Get style for status idle
    pub fn status_idle() -> Style {
        Style::default().fg(text::MUTED)
    }

    /// Get style for status error
    pub fn status_error() -> Style {
        Style::default().fg(accent::RED)
    }

    /// Get style for status completed
    pub fn status_completed() -> Style {
        Style::default().fg(accent::GREEN)
    }

    /// Get style for git modified files
    pub fn git_modified() -> Style {
        Style::default().fg(accent::YELLOW)
    }

    /// Get style for git staged files
    pub fn git_staged() -> Style {
        Style::default().fg(accent::GREEN)
    }

    /// Get style for git untracked files
    pub fn git_untracked() -> Style {
        Style::default().fg(accent::CYAN)
    }

    /// Get style for directory icons
    pub fn directory_icon() -> Style {
        Style::default().fg(accent::PEACH)
    }

    /// Get style for file icons
    pub fn file_icon() -> Style {
        Style::default().fg(text::SECONDARY)
    }

    /// Get style for command prompt
    pub fn command_prompt() -> Style {
        Style::default().fg(text::SECONDARY)
    }

    /// Get style for input cursor
    pub fn input_cursor() -> Style {
        Style::default().fg(text::PRIMARY)
    }

    /// Get style for key hints in status bar
    pub fn key_hint() -> Style {
        Style::default().fg(text::SECONDARY)
    }

    /// Get style for key descriptions
    pub fn key_desc() -> Style {
        Style::default().fg(text::MUTED)
    }

    /// Get style for timestamps
    pub fn timestamp() -> Style {
        Style::default().fg(text::MUTED)
    }

    /// Get style for separator
    pub fn separator() -> Style {
        Style::default().fg(border::DEFAULT)
    }

    /// Get style for fuzzy match highlights
    pub fn fuzzy_match() -> Style {
        Style::default().fg(accent::CYAN).add_modifier(Modifier::BOLD)
    }

    /// Get style for overlay background
    pub fn overlay_bg() -> Style {
        Style::default().bg(bg::PANEL)
    }

    /// Get status bar background style
    pub fn status_bar_bg() -> Style {
        Style::default().bg(bg::STATUS).fg(text::PRIMARY)
    }

    /// Get left accent bar for selected items
    pub fn accent_bar_span() -> Span<'static> {
        Span::styled("▎", Style::default().fg(accent::CYAN))
    }

    /// Style for "+ New workspace" links
    pub fn link_style() -> Style {
        Style::default().fg(accent::CYAN)
    }

    /// Style for workspace names in sidebar
    pub fn workspace_name() -> Style {
        Style::default().fg(text::PRIMARY)
    }

    /// Style for workspace sub-items
    pub fn workspace_item() -> Style {
        Style::default().fg(text::SECONDARY)
    }
}
