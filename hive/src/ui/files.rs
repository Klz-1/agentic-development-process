//! Files panel - displays file tree with git status

use crate::app::{App, FocusedPanel};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState};

/// Mock file entry for demonstration
struct MockFileEntry {
    name: &'static str,
    is_dir: bool,
    is_expanded: bool,
    depth: usize,
    git_status: Option<GitDisplayStatus>,
    is_last: bool, // Is this the last item at its depth level
}

#[derive(Clone, Copy)]
enum GitDisplayStatus {
    Modified,
    Staged,
    Untracked,
}

impl MockFileEntry {
    fn to_list_item(&self) -> ListItem<'static> {
        // Build tree indentation
        let mut prefix = String::new();
        for _ in 0..self.depth.saturating_sub(1) {
            prefix.push_str("│   ");
        }

        // Add connector for non-root items
        if self.depth > 0 {
            if self.is_last {
                prefix.push_str("└── ");
            } else {
                prefix.push_str("├── ");
            }
        }

        // Build the icon for directories/files
        let icon = if self.is_dir {
            if self.is_expanded {
                "📂 " // Open folder
            } else {
                "📁 " // Closed folder
            }
        } else {
            "📄 " // File
        };

        // Git status indicator
        let git_text = match self.git_status {
            Some(GitDisplayStatus::Modified) => " [M]",
            Some(GitDisplayStatus::Staged) => " [S]",
            Some(GitDisplayStatus::Untracked) => " [+]",
            None => "",
        };

        let git_style = match self.git_status {
            Some(GitDisplayStatus::Modified) => Style::default().fg(Color::Yellow),
            Some(GitDisplayStatus::Staged) => Style::default().fg(Color::Green),
            Some(GitDisplayStatus::Untracked) => Style::default().fg(Color::Cyan),
            None => Style::default(),
        };

        // Build the full display text
        let display = format!("{}{}{}{}", prefix, icon, self.name, git_text);

        // Color the different parts
        let styled_line = if self.git_status.is_some() {
            // Find where git status starts
            let git_start = display.len() - git_text.len();
            Line::from(vec![
                Span::styled(display[..prefix.len()].to_string(), Style::default().fg(Color::DarkGray)),
                Span::styled(icon.to_string(), if self.is_dir { Style::default().fg(Color::Blue) } else { Style::default() }),
                Span::raw(self.name.to_string()),
                Span::styled(display[git_start..].to_string(), git_style),
            ])
        } else {
            Line::from(vec![
                Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                Span::styled(icon.to_string(), if self.is_dir { Style::default().fg(Color::Blue) } else { Style::default() }),
                Span::raw(self.name.to_string()),
            ])
        };

        ListItem::new(styled_line)
    }
}

pub struct FilesPanel<'a> {
    app: &'a App,
}

impl<'a> FilesPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let is_focused = self.app.focused_panel == FocusedPanel::Files;

        let border_style = if is_focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        // Calculate scroll indicator
        let total_items = self.app.file_count;
        let visible_height = area.height.saturating_sub(2) as usize;
        let scroll_indicator = if total_items > visible_height {
            format!(" [{}/{}] ", self.app.selected_file + 1, total_items)
        } else {
            String::new()
        };

        let block = Block::default()
            .title(" Files ")
            .title_bottom(Line::from(scroll_indicator).right_aligned())
            .borders(Borders::ALL)
            .border_style(border_style);

        // Mock file tree data
        let files = vec![
            MockFileEntry { name: "~/projects/webapp", is_dir: true, is_expanded: true, depth: 0, git_status: None, is_last: true },
            MockFileEntry { name: "src", is_dir: true, is_expanded: true, depth: 1, git_status: None, is_last: false },
            MockFileEntry { name: "main.rs", is_dir: false, is_expanded: false, depth: 2, git_status: None, is_last: false },
            MockFileEntry { name: "lib.rs", is_dir: false, is_expanded: false, depth: 2, git_status: Some(GitDisplayStatus::Modified), is_last: false },
            MockFileEntry { name: "utils.rs", is_dir: false, is_expanded: false, depth: 2, git_status: Some(GitDisplayStatus::Modified), is_last: true },
            MockFileEntry { name: "tests", is_dir: true, is_expanded: false, depth: 1, git_status: None, is_last: false },
            MockFileEntry { name: "Cargo.toml", is_dir: false, is_expanded: false, depth: 1, git_status: None, is_last: false },
            MockFileEntry { name: "README.md", is_dir: false, is_expanded: false, depth: 1, git_status: Some(GitDisplayStatus::Staged), is_last: false },
            MockFileEntry { name: "CHANGELOG.md", is_dir: false, is_expanded: false, depth: 1, git_status: Some(GitDisplayStatus::Untracked), is_last: true },
        ];

        // Build list items
        let items: Vec<ListItem> = files
            .iter()
            .map(|f| f.to_list_item())
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray)
            )
            .highlight_symbol("▶ ");

        let mut state = ListState::default();
        state.select(Some(self.app.selected_file));

        frame.render_stateful_widget(list, area, &mut state);

        // Render scrollbar if content exceeds visible area
        if total_items > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"))
                .track_symbol(Some("│"))
                .thumb_symbol("█");

            let mut scrollbar_state = ScrollbarState::new(total_items)
                .position(self.app.files_scroll);

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
