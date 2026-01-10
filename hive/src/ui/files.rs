//! Files panel - displays file tree with git status

use crate::app::{App, FocusedPanel};
use crate::files::{FileKind, GitStatus};
use crate::theme::{self, Theme};
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

    /// Check if we're viewing hive's own directory (inception!)
    fn is_viewing_self(&self) -> bool {
        self.app
            .file_tree
            .root
            .to_string_lossy()
            .to_lowercase()
            .contains("/hive")
    }

    pub fn render(self, frame: &mut Frame, area: Rect) {
        let is_focused = self.app.focused_panel == FocusedPanel::Files;

        let total_items = self.app.file_count;
        let visible_height = area.height.saturating_sub(2) as usize;
        let scroll_indicator = if total_items > visible_height {
            format!(" [{}/{}] ", self.app.selected_file + 1, total_items)
        } else {
            String::new()
        };

        let hidden_indicator = if self.app.file_tree.show_hidden {
            " [.*] "
        } else {
            ""
        };

        let block = Block::default()
            .title(format!(" Files{}", hidden_indicator))
            .title_style(Theme::title_style(is_focused))
            .title_bottom(Line::from(Span::styled(scroll_indicator, Style::default().fg(theme::text::MUTED))).right_aligned())
            .borders(Borders::ALL)
            .border_style(Theme::border_style(is_focused))
            .style(Theme::panel_bg());

        // Check for inception
        let items: Vec<ListItem> = if self.is_viewing_self() {
            vec![
                ListItem::new(Line::from("")),
                ListItem::new(Line::from(Span::styled(
                    "  🪞 RECURSION DETECTED 🪞",
                    Style::default().fg(theme::accent::CYAN).bold(),
                ))),
                ListItem::new(Line::from("")),
                ListItem::new(Line::from(Span::styled(
                    "  Hive cannot browse itself.",
                    Style::default().fg(theme::text::SECONDARY),
                ))),
                ListItem::new(Line::from("")),
                ListItem::new(Line::from(Span::styled(
                    "  It's turtles all the way down,",
                    Style::default().fg(theme::text::MUTED).italic(),
                ))),
                ListItem::new(Line::from(Span::styled(
                    "  but we stopped here.",
                    Style::default().fg(theme::text::MUTED).italic(),
                ))),
                ListItem::new(Line::from("")),
                ListItem::new(Line::from(Span::styled(
                    "  🐢 🐢 🐢",
                    Style::default().fg(theme::accent::GREEN),
                ))),
            ]
        } else {
            self.app
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

                    let mut prefix = String::new();
                    for _ in 0..entry.depth.saturating_sub(1) {
                        prefix.push_str("│   ");
                    }

                    if entry.depth > 0 {
                        let is_last = self.is_last_at_depth(idx, entry.depth);
                        if is_last {
                            prefix.push_str("└── ");
                        } else {
                            prefix.push_str("├── ");
                        }
                    }

                    let is_expanded = self.app.file_tree.is_expanded(&entry.path);
                    let icon = match entry.kind {
                        FileKind::Directory => {
                            if is_expanded { "▾ " } else { "▸ " }
                        }
                        FileKind::File => "  ",
                    };

                    let (git_text, git_style) = match entry.git_status {
                        Some(GitStatus::Modified) => (" [M]", Theme::git_modified()),
                        Some(GitStatus::Staged) => (" [S]", Theme::git_staged()),
                        Some(GitStatus::Untracked) => (" [+]", Theme::git_untracked()),
                        Some(GitStatus::Conflicted) => (" [!]", Theme::status_error()),
                        None => ("", Style::default()),
                    };

                    let styled_line = Line::from(vec![
                        Span::styled(prefix, Style::default().fg(theme::text::MUTED)),
                        Span::styled(
                            icon.to_string(),
                            if entry.kind == FileKind::Directory {
                                Theme::directory_icon()
                            } else {
                                Style::default()
                            },
                        ),
                        Span::styled(name.to_string(), Style::default().fg(theme::text::PRIMARY)),
                        Span::styled(git_text.to_string(), git_style),
                    ]);

                    ListItem::new(styled_line)
                })
                .collect()
        };

        let list = List::new(items)
            .block(block)
            .highlight_style(Theme::highlight_style())
            .highlight_symbol("▸ ");

        let mut state = ListState::default();
        state.select(Some(self.app.selected_file));

        frame.render_stateful_widget(list, area, &mut state);

        if total_items > visible_height {
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some(" "))
                .thumb_symbol("▐")
                .style(Theme::scrollbar_style())
                .thumb_style(Theme::scrollbar_thumb_style());

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

    fn is_last_at_depth(&self, idx: usize, depth: usize) -> bool {
        for i in (idx + 1)..self.app.file_tree.entries.len() {
            let next_entry = &self.app.file_tree.entries[i];
            if next_entry.depth < depth {
                return true;
            }
            if next_entry.depth == depth {
                return false;
            }
        }
        true
    }
}
