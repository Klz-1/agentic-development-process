//! Files panel - displays file tree with git status

use crate::app::{App, FocusedPanel};
use crate::files::{FileKind, GitStatus};
use ratatui::prelude::*;
use ratatui::widgets::{
    Block, Borders, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
};

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

        // Show hidden files indicator
        let hidden_indicator = if self.app.file_tree.show_hidden {
            " [.*] "
        } else {
            ""
        };

        let block = Block::default()
            .title(format!(" Files{}", hidden_indicator))
            .title_bottom(Line::from(scroll_indicator).right_aligned())
            .borders(Borders::ALL)
            .border_style(border_style);

        // Build list items from real file tree
        let items: Vec<ListItem> = self
            .app
            .file_tree
            .entries
            .iter()
            .enumerate()
            .map(|(idx, entry)| {
                let name = entry
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();

                // Build tree indentation
                let mut prefix = String::new();
                for _ in 0..entry.depth.saturating_sub(1) {
                    prefix.push_str("│   ");
                }

                // Add connector for non-root items
                if entry.depth > 0 {
                    // Check if this is the last item at this depth
                    let is_last = self.is_last_at_depth(idx, entry.depth);
                    if is_last {
                        prefix.push_str("└── ");
                    } else {
                        prefix.push_str("├── ");
                    }
                }

                // Build the icon for directories/files
                let is_expanded = self.app.file_tree.is_expanded(&entry.path);
                let icon = match entry.kind {
                    FileKind::Directory => {
                        if is_expanded {
                            "📂 " // Open folder
                        } else {
                            "📁 " // Closed folder
                        }
                    }
                    FileKind::File => "📄 ", // File
                };

                // Git status indicator
                let (git_text, git_style) = match entry.git_status {
                    Some(GitStatus::Modified) => {
                        (" [M]", Style::default().fg(Color::Yellow))
                    }
                    Some(GitStatus::Staged) => (" [S]", Style::default().fg(Color::Green)),
                    Some(GitStatus::Untracked) => (" [+]", Style::default().fg(Color::Cyan)),
                    Some(GitStatus::Conflicted) => (" [!]", Style::default().fg(Color::Red)),
                    None => ("", Style::default()),
                };

                // Build styled spans
                let styled_line = Line::from(vec![
                    Span::styled(prefix, Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        icon.to_string(),
                        if entry.kind == FileKind::Directory {
                            Style::default().fg(Color::Blue)
                        } else {
                            Style::default()
                        },
                    ),
                    Span::raw(name.to_string()),
                    Span::styled(git_text.to_string(), git_style),
                ]);

                ListItem::new(styled_line)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray),
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

            let mut scrollbar_state =
                ScrollbarState::new(total_items).position(self.app.files_scroll);

            let scrollbar_area = Rect {
                x: area.x + area.width - 1,
                y: area.y + 1,
                width: 1,
                height: area.height.saturating_sub(2),
            };

            frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }

    /// Check if the entry at `idx` is the last one at the given depth
    fn is_last_at_depth(&self, idx: usize, depth: usize) -> bool {
        // Look ahead to see if there are any more items at the same depth
        for i in (idx + 1)..self.app.file_tree.entries.len() {
            let next_entry = &self.app.file_tree.entries[i];
            if next_entry.depth < depth {
                // We went up in the tree, so this was the last
                return true;
            }
            if next_entry.depth == depth {
                // There's another item at the same depth
                return false;
            }
            // next_entry.depth > depth means we're in a subdirectory, continue
        }
        // No more items, so this is the last
        true
    }
}
