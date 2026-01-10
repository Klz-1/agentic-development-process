//! Fuzzy finder overlay - quick file/command search
//!
//! Uses nucleo for high-performance fuzzy matching of files, commands, and sessions.

use nucleo::{Config as NucleoConfig, Matcher, Utf32Str};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};
use std::path::PathBuf;

/// State for the fuzzy finder
#[derive(Debug)]
pub struct FuzzyFinderState {
    /// Whether the fuzzy finder is open
    pub open: bool,
    /// Current input query
    pub query: String,
    /// Selected result index
    pub selected: usize,
    /// Search results
    pub results: Vec<FuzzyResult>,
    /// All searchable items
    items: Vec<FuzzyItem>,
    /// Nucleo matcher
    matcher: Matcher,
}

impl Default for FuzzyFinderState {
    fn default() -> Self {
        Self {
            open: false,
            query: String::new(),
            selected: 0,
            results: Vec::new(),
            items: Vec::new(),
            matcher: Matcher::new(NucleoConfig::DEFAULT),
        }
    }
}

/// Internal searchable item
#[derive(Debug, Clone)]
struct FuzzyItem {
    text: String,
    kind: FuzzyResultKind,
    path: Option<PathBuf>,
}

/// A single fuzzy search result
#[derive(Debug, Clone)]
pub struct FuzzyResult {
    /// Display text
    pub display: String,
    /// Score (higher is better)
    pub score: u32,
    /// Kind of result
    pub kind: FuzzyResultKind,
    /// Path for file results
    pub path: Option<PathBuf>,
    /// Match indices for highlighting
    pub indices: Vec<u32>,
}

/// Type of fuzzy result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuzzyResultKind {
    File,
    Command,
    Session,
}

impl FuzzyFinderState {
    /// Create a new fuzzy finder with items to search
    pub fn new() -> Self {
        Self::default()
    }

    /// Add files to the searchable items
    pub fn add_files(&mut self, files: Vec<PathBuf>) {
        for path in files {
            let display = path.to_string_lossy().to_string();
            self.items.push(FuzzyItem {
                text: display,
                kind: FuzzyResultKind::File,
                path: Some(path),
            });
        }
    }

    /// Add sessions to the searchable items
    pub fn add_sessions(&mut self, sessions: Vec<String>) {
        for name in sessions {
            self.items.push(FuzzyItem {
                text: name,
                kind: FuzzyResultKind::Session,
                path: None,
            });
        }
    }

    /// Add commands to the searchable items
    pub fn add_commands(&mut self, commands: Vec<&'static str>) {
        for cmd in commands {
            self.items.push(FuzzyItem {
                text: cmd.to_string(),
                kind: FuzzyResultKind::Command,
                path: None,
            });
        }
    }

    /// Open the fuzzy finder
    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.update_results();
    }

    /// Close the fuzzy finder
    pub fn close(&mut self) {
        self.open = false;
        self.query.clear();
        self.selected = 0;
        self.results.clear();
    }

    /// Handle character input
    pub fn input(&mut self, c: char) {
        self.query.push(c);
        self.update_results();
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        self.query.pop();
        self.update_results();
    }

    /// Move selection up
    pub fn select_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.results.is_empty() {
            self.selected = (self.selected + 1).min(self.results.len() - 1);
        }
    }

    /// Get the currently selected result
    pub fn get_selected(&self) -> Option<&FuzzyResult> {
        self.results.get(self.selected)
    }

    /// Update results using nucleo fuzzy matching
    fn update_results(&mut self) {
        self.results.clear();
        self.selected = 0;

        if self.query.is_empty() {
            // Show all items when query is empty (limited)
            for item in self.items.iter().take(20) {
                self.results.push(FuzzyResult {
                    display: item.text.clone(),
                    score: 0,
                    kind: item.kind,
                    path: item.path.clone(),
                    indices: Vec::new(),
                });
            }
            return;
        }

        // Convert query to Utf32Str for nucleo
        let mut query_buf = Vec::new();
        let query = Utf32Str::new(&self.query, &mut query_buf);

        // Score each item
        let mut scored: Vec<(u32, Vec<u32>, &FuzzyItem)> = Vec::new();

        for item in &self.items {
            let mut haystack_buf = Vec::new();
            let haystack = Utf32Str::new(&item.text, &mut haystack_buf);

            let mut indices = Vec::new();
            if let Some(score) = self.matcher.fuzzy_indices(haystack, query, &mut indices) {
                scored.push((score as u32, indices, item));
            }
        }

        // Sort by score (highest first)
        scored.sort_by(|a, b| b.0.cmp(&a.0));

        // Take top results
        for (score, indices, item) in scored.into_iter().take(15) {
            self.results.push(FuzzyResult {
                display: item.text.clone(),
                score,
                kind: item.kind,
                path: item.path.clone(),
                indices,
            });
        }
    }

    /// Clear all items
    pub fn clear_items(&mut self) {
        self.items.clear();
        self.results.clear();
    }
}

/// Fuzzy finder overlay widget
pub struct FuzzyFinderOverlay<'a> {
    query: &'a str,
    results: Option<&'a [FuzzyResult]>,
    selected: usize,
}

impl<'a> FuzzyFinderOverlay<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            results: None,
            selected: 0,
        }
    }

    /// Create from a FuzzyFinderState
    pub fn from_state(state: &'a FuzzyFinderState) -> Self {
        Self {
            query: &state.query,
            results: Some(&state.results),
            selected: state.selected,
        }
    }

    pub fn render(self, frame: &mut Frame) {
        let area = centered_rect(50, 60, frame.area());

        // Clear the background
        frame.render_widget(Clear, area);

        let block = Block::default()
            .title(" Find ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Split inner area: input line + results
        let chunks = Layout::vertical([
            Constraint::Length(1), // Input line
            Constraint::Length(1), // Separator
            Constraint::Min(1),    // Results
            Constraint::Length(1), // Hint line
        ])
        .split(inner);

        // Render input line with cursor
        let input = Paragraph::new(Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::Cyan)),
            Span::raw(self.query),
            Span::styled("_", Style::default().add_modifier(Modifier::SLOW_BLINK)),
        ]));
        frame.render_widget(input, chunks[0]);

        // Render separator
        let separator = Paragraph::new("─".repeat(chunks[1].width as usize))
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(separator, chunks[1]);

        // Render results
        if let Some(results) = self.results {
            self.render_results(frame, chunks[2], results);
        } else {
            // Fallback to placeholder results
            let results = self.get_placeholder_results();
            self.render_placeholder_results(frame, chunks[2], &results);
        }

        // Render hint line
        let hint = Paragraph::new(Line::from(vec![
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::styled(" navigate  ", Style::default().fg(Color::DarkGray)),
            Span::styled("enter", Style::default().fg(Color::Yellow)),
            Span::styled(" select  ", Style::default().fg(Color::DarkGray)),
            Span::styled("esc", Style::default().fg(Color::Yellow)),
            Span::styled(" cancel", Style::default().fg(Color::DarkGray)),
        ]));
        frame.render_widget(hint, chunks[3]);
    }

    fn render_results(&self, frame: &mut Frame, area: Rect, results: &[FuzzyResult]) {
        if results.is_empty() && !self.query.is_empty() {
            let no_results = Paragraph::new("  No results")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(no_results, area);
            return;
        }

        if results.is_empty() {
            let hint = Paragraph::new("  Type to search files, commands, sessions...")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(hint, area);
            return;
        }

        let items: Vec<ListItem> = results
            .iter()
            .enumerate()
            .map(|(i, result)| {
                let icon = match result.kind {
                    FuzzyResultKind::File => "📄",
                    FuzzyResultKind::Command => "⌘ ",
                    FuzzyResultKind::Session => "● ",
                };

                let style = if i == self.selected {
                    Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };

                // Build display with highlighted matches
                let display = if !result.indices.is_empty() && i == self.selected {
                    // Highlight matched characters
                    let mut spans = vec![Span::styled(format!("{} ", icon), style)];
                    let chars: Vec<char> = result.display.chars().collect();
                    let indices_set: std::collections::HashSet<u32> =
                        result.indices.iter().cloned().collect();

                    for (idx, ch) in chars.iter().enumerate() {
                        if indices_set.contains(&(idx as u32)) {
                            spans.push(Span::styled(
                                ch.to_string(),
                                style.fg(Color::Yellow).add_modifier(Modifier::BOLD),
                            ));
                        } else {
                            spans.push(Span::styled(ch.to_string(), style));
                        }
                    }
                    Line::from(spans)
                } else {
                    Line::from(vec![
                        Span::styled(format!("{} ", icon), style),
                        Span::styled(&result.display, style),
                    ])
                };

                ListItem::new(display)
            })
            .collect();

        let list = List::new(items);
        frame.render_widget(list, area);
    }

    fn render_placeholder_results(
        &self,
        frame: &mut Frame,
        area: Rect,
        results: &[(&str, FuzzyResultKind)],
    ) {
        if results.is_empty() && !self.query.is_empty() {
            let no_results = Paragraph::new("  No results")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(no_results, area);
            return;
        }

        if results.is_empty() {
            let hint = Paragraph::new("  Type to search files, commands, sessions...")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(hint, area);
            return;
        }

        let items: Vec<ListItem> = results
            .iter()
            .enumerate()
            .map(|(i, (name, kind))| {
                let icon = match kind {
                    FuzzyResultKind::File => "📄",
                    FuzzyResultKind::Command => "⌘ ",
                    FuzzyResultKind::Session => "● ",
                };
                let style = if i == 0 {
                    Style::default().bg(Color::Rgb(50, 50, 50)).fg(Color::White)
                } else {
                    Style::default().fg(Color::Gray)
                };
                ListItem::new(format!("{} {}", icon, name)).style(style)
            })
            .collect();

        let list = List::new(items);
        frame.render_widget(list, area);
    }

    /// Get placeholder results based on query (fallback)
    fn get_placeholder_results(&self) -> Vec<(&'static str, FuzzyResultKind)> {
        if self.query.is_empty() {
            return vec![];
        }

        let placeholders = vec![
            ("src/main.rs", FuzzyResultKind::File),
            ("src/app.rs", FuzzyResultKind::File),
            ("src/ui/mod.rs", FuzzyResultKind::File),
            ("src/ui/sessions.rs", FuzzyResultKind::File),
            ("src/ui/output.rs", FuzzyResultKind::File),
            ("src/ui/files.rs", FuzzyResultKind::File),
            (":attach", FuzzyResultKind::Command),
            (":quit", FuzzyResultKind::Command),
            (":help", FuzzyResultKind::Command),
            ("webapp-agent", FuzzyResultKind::Session),
            ("api-refactor", FuzzyResultKind::Session),
        ];

        placeholders
            .into_iter()
            .filter(|(name, _)| name.to_lowercase().contains(&self.query.to_lowercase()))
            .take(10)
            .collect()
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
