//! Fuzzy finder overlay - quick file/command search
//!
//! Provides a fuzzy search interface for files, commands, and sessions.

#![allow(dead_code)]

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, Paragraph};

/// State for the fuzzy finder
#[derive(Debug, Default)]
pub struct FuzzyFinderState {
    /// Whether the fuzzy finder is open
    pub open: bool,
    /// Current input query
    pub query: String,
    /// Selected result index
    pub selected: usize,
    /// Search results
    pub results: Vec<FuzzyResult>,
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
}

/// Type of fuzzy result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FuzzyResultKind {
    File,
    Command,
    Session,
}

impl FuzzyFinderState {
    /// Open the fuzzy finder
    pub fn open(&mut self) {
        self.open = true;
        self.query.clear();
        self.selected = 0;
        self.results.clear();
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

    /// Update results based on current query (stub - will be connected to nucleo later)
    fn update_results(&mut self) {
        // Stub implementation - just show some placeholder results
        self.results.clear();
        self.selected = 0;

        if self.query.is_empty() {
            return;
        }

        // Placeholder results for demonstration
        let placeholders = vec![
            ("src/main.rs", FuzzyResultKind::File),
            ("src/app.rs", FuzzyResultKind::File),
            ("src/ui/mod.rs", FuzzyResultKind::File),
            (":attach", FuzzyResultKind::Command),
            (":quit", FuzzyResultKind::Command),
        ];

        for (name, kind) in placeholders {
            if name.to_lowercase().contains(&self.query.to_lowercase()) {
                self.results.push(FuzzyResult {
                    display: name.to_string(),
                    score: 100,
                    kind,
                });
            }
        }
    }
}

/// Fuzzy finder overlay widget
pub struct FuzzyFinderOverlay<'a> {
    query: &'a str,
}

impl<'a> FuzzyFinderOverlay<'a> {
    pub fn new(query: &'a str) -> Self {
        Self { query }
    }

    /// Create from a FuzzyFinderState
    #[allow(dead_code)]
    pub fn from_state(state: &'a FuzzyFinderState) -> Self {
        Self { query: &state.query }
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
        ])
        .split(inner);

        // Render input line with cursor
        let input = Paragraph::new(format!("> {}_", self.query))
            .style(Style::default().fg(Color::White));
        frame.render_widget(input, chunks[0]);

        // Render separator
        let separator = Paragraph::new("─".repeat(chunks[1].width as usize))
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(separator, chunks[1]);

        // Generate placeholder results based on query (stub)
        let results = self.get_placeholder_results();

        // Render results
        let items: Vec<ListItem> = results
            .iter()
            .enumerate()
            .map(|(i, (name, kind))| {
                let icon = match kind {
                    FuzzyResultKind::File => " ",
                    FuzzyResultKind::Command => " ",
                    FuzzyResultKind::Session => " ",
                };
                let style = if i == 0 {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}", icon, name)).style(style)
            })
            .collect();

        if items.is_empty() && !self.query.is_empty() {
            let no_results = Paragraph::new("  No results")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(no_results, chunks[2]);
        } else if items.is_empty() {
            let hint = Paragraph::new("  Type to search files, commands, sessions...")
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(hint, chunks[2]);
        } else {
            let list = List::new(items);
            frame.render_widget(list, chunks[2]);
        }
    }

    /// Get placeholder results based on query (stub implementation)
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
