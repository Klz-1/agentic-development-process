//! Application state and main event loop

use crate::config::Config;
use crate::event::{Event, EventHandler};
use crate::files::{get_git_status, FileKind, FileTree};
use crate::tmux::{CaptureHandle, OutputCapture, Session, TmuxClient};
use crate::ui;
use crate::utils::RingBuffer;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Which panel currently has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusedPanel {
    #[default]
    Sessions,
    Output,
    Files,
}

impl FocusedPanel {
    pub fn next(self) -> Self {
        match self {
            Self::Sessions => Self::Output,
            Self::Output => Self::Files,
            Self::Files => Self::Sessions,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Sessions => Self::Files,
            Self::Output => Self::Sessions,
            Self::Files => Self::Output,
        }
    }
}

/// Panel resize state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizeState {
    #[default]
    None,
    /// Dragging the border between sessions and output panels
    DraggingSessionsOutput,
    /// Dragging the border between output and files panels
    DraggingOutputFiles,
}

/// Main application state
pub struct App {
    /// Application configuration
    #[allow(dead_code)]
    pub config: Config,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Currently focused panel
    pub focused_panel: FocusedPanel,
    /// Selected session index
    pub selected_session: usize,
    /// Selected file index
    pub selected_file: usize,
    /// Output scroll position
    pub output_scroll: usize,
    /// Whether output auto-scrolls
    pub output_auto_scroll: bool,
    /// Whether help overlay is visible
    pub show_help: bool,
    /// Whether fuzzy finder is open
    pub show_fuzzy_finder: bool,
    /// Fuzzy finder input text
    pub fuzzy_input: String,
    /// Panel width percentages (sessions, output, files)
    pub panel_widths: (u16, u16, u16),
    /// Current resize state
    pub resize_state: ResizeState,
    /// Sessions panel scroll offset
    pub sessions_scroll: usize,
    /// Files panel scroll offset
    pub files_scroll: usize,
    /// Total number of sessions (for scroll bounds)
    pub session_count: usize,
    /// Total number of files (for scroll bounds)
    pub file_count: usize,
    /// Total output lines (for scroll bounds)
    pub output_line_count: usize,
    /// Last known terminal width
    pub terminal_width: u16,
    /// Real tmux sessions
    pub sessions: Vec<Session>,
    /// Tmux client for interaction
    pub tmux_client: TmuxClient,
    /// Output capture manager
    pub output_capture: OutputCapture,
    /// Output buffer for selected session
    pub output_buffer: Arc<Mutex<RingBuffer<String>>>,
    /// Cached output lines for rendering (updated from buffer)
    pub output_lines: Vec<String>,
    /// Command input buffer
    pub command_input: String,
    /// Whether we're in command input mode
    pub command_mode: bool,
    /// File tree for the files panel
    pub file_tree: FileTree,
    /// File to open in editor (set when 'e' is pressed, handled after TUI exit)
    pub pending_editor_file: Option<PathBuf>,
    /// Alert state - true when there's an error that needs attention
    pub alert_active: bool,
    /// Last alert message
    pub alert_message: Option<String>,
    /// Handle to current output capture task
    capture_handle: Option<CaptureHandle>,
    /// Name of session currently being captured
    captured_session: Option<String>,
}

impl App {
    pub fn new(config: Config) -> Self {
        // Create file tree for current directory
        let mut file_tree = FileTree::new(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        // Update git status for file tree
        if let Ok(status) = get_git_status(&file_tree.root) {
            file_tree.update_git_status(&status);
        }

        let file_count = file_tree.entries.len();

        Self {
            config,
            should_quit: false,
            focused_panel: FocusedPanel::default(),
            selected_session: 0,
            selected_file: 0,
            output_scroll: 0,
            output_auto_scroll: false, // Start from top
            show_help: false,
            show_fuzzy_finder: false,
            fuzzy_input: String::new(),
            panel_widths: (20, 40, 40), // Default percentages
            resize_state: ResizeState::None,
            sessions_scroll: 0,
            files_scroll: 0,
            session_count: 0,
            file_count,
            output_line_count: 0,
            terminal_width: 80,
            sessions: Vec::new(),
            tmux_client: TmuxClient::new(),
            output_capture: OutputCapture::new(1000),
            output_buffer: Arc::new(Mutex::new(RingBuffer::new(1000))),
            output_lines: Vec::new(),
            command_input: String::new(),
            command_mode: false,
            file_tree,
            pending_editor_file: None,
            alert_active: false,
            alert_message: None,
            capture_handle: None,
            captured_session: None,
        }
    }

    /// Get the currently selected file path
    pub fn selected_file_path(&self) -> Option<PathBuf> {
        self.file_tree.entries.get(self.selected_file).map(|e| e.path.clone())
    }

    /// Open the selected file in an external editor
    /// This sets the pending_editor_file which is handled after TUI cleanup
    pub fn open_selected_in_editor(&mut self) {
        if let Some(entry) = self.file_tree.entries.get(self.selected_file) {
            if entry.kind == FileKind::File {
                self.pending_editor_file = Some(entry.path.clone());
                self.should_quit = true; // Exit TUI to open editor
            }
        }
    }

    /// Trigger an alert (visual and optional sound)
    pub fn trigger_alert(&mut self, message: &str) {
        self.alert_active = true;
        self.alert_message = Some(message.to_string());

        // Ring terminal bell if sound alerts are enabled
        if self.config.alerts.sound {
            print!("\x07"); // ASCII BEL character
        }
    }

    /// Clear the current alert
    pub fn clear_alert(&mut self) {
        self.alert_active = false;
        self.alert_message = None;
    }

    /// Set the file tree root to a new path and refresh
    pub fn set_file_root(&mut self, path: PathBuf) {
        // Only update if the path is different
        if self.file_tree.root != path {
            self.file_tree = FileTree::new(path);
            self.update_file_tree_git_status();
            self.file_count = self.file_tree.entries.len();
            self.selected_file = 0;
            self.files_scroll = 0;
        }
    }

    /// Sync file tree to the currently selected session's project root
    pub fn sync_file_tree_to_session(&mut self) {
        if let Some(session) = self.sessions.get(self.selected_session) {
            let project_root = session.project_root.clone();
            self.set_file_root(project_root);
        }
    }

    /// Update git status for all file tree entries
    pub fn update_file_tree_git_status(&mut self) {
        if let Ok(status) = get_git_status(&self.file_tree.root) {
            self.file_tree.update_git_status(&status);
        }
    }

    /// Toggle expansion of the selected directory
    pub fn toggle_file_expand(&mut self) {
        if let Some(entry) = self.file_tree.entries.get(self.selected_file) {
            if entry.kind == FileKind::Directory {
                let path = entry.path.clone();
                self.file_tree.toggle_expand(&path);
                self.update_file_tree_git_status();
                self.file_count = self.file_tree.entries.len();
            }
        }
    }

    /// Toggle showing hidden files
    pub fn toggle_hidden_files(&mut self) {
        self.file_tree.toggle_hidden();
        self.update_file_tree_git_status();
        self.file_count = self.file_tree.entries.len();
        // Clamp selected file if needed
        if self.selected_file >= self.file_count && self.file_count > 0 {
            self.selected_file = self.file_count - 1;
        }
    }

    /// Refresh session list from tmux
    pub async fn refresh_sessions(&mut self) -> Result<()> {
        let old_session_count = self.sessions.len();
        self.sessions = self.tmux_client.list_sessions().await?;
        self.session_count = self.sessions.len();

        // Clamp selected session if sessions were removed
        if self.selected_session >= self.session_count && self.session_count > 0 {
            self.selected_session = self.session_count - 1;
        }

        // Sync file tree to selected session if this is first load or sessions changed
        if old_session_count == 0 && self.session_count > 0 {
            self.sync_to_session();
        }

        Ok(())
    }

    /// Refresh output from buffer
    pub async fn refresh_output(&mut self) {
        let buffer = self.output_buffer.lock().await;
        self.output_lines = buffer.iter().cloned().collect();
        self.output_line_count = self.output_lines.len();

        // Auto-scroll to bottom if enabled
        if self.output_auto_scroll && self.output_line_count > 0 {
            self.output_scroll = self.output_line_count.saturating_sub(1);
        }
    }

    /// Get the currently selected session
    pub fn selected_session_data(&self) -> Option<&Session> {
        self.sessions.get(self.selected_session)
    }

    /// Send a command to the selected session
    #[allow(dead_code)]
    pub async fn send_command(&mut self) -> Result<()> {
        if self.command_input.is_empty() {
            return Ok(());
        }

        if let Some(session) = self.selected_session_data() {
            self.tmux_client
                .send_keys(&session.name, &self.command_input)
                .await?;
            self.command_input.clear();
        }

        Ok(())
    }

    /// Send interrupt (Ctrl+C) to the selected session
    #[allow(dead_code)]
    pub async fn send_interrupt(&self) -> Result<()> {
        if let Some(session) = self.selected_session_data() {
            self.tmux_client.send_interrupt(&session.name).await?;
        }
        Ok(())
    }

    /// Attach to the selected session (exits TUI)
    #[allow(dead_code)]
    pub fn attach_to_session(&self) -> Result<()> {
        if let Some(session) = self.selected_session_data() {
            self.tmux_client.attach(&session.name)?;
        }
        Ok(())
    }

    /// Focus a specific panel
    #[allow(dead_code)]
    pub fn focus_panel(&mut self, panel: FocusedPanel) {
        self.focused_panel = panel;
    }

    /// Set panel widths ensuring minimum sizes
    #[allow(dead_code)]
    pub fn set_panel_widths(&mut self, sessions: u16, output: u16, files: u16) {
        const MIN_WIDTH: u16 = 10;

        // Ensure minimum widths
        let sessions = sessions.max(MIN_WIDTH);
        let output = output.max(MIN_WIDTH);
        let files = files.max(MIN_WIDTH);

        // Normalize to 100%
        let total = sessions + output + files;
        if total > 0 {
            self.panel_widths = (
                (sessions * 100) / total,
                (output * 100) / total,
                (files * 100) / total,
            );
        }
    }

    /// Update panel width based on drag position
    pub fn update_resize(&mut self, x: u16) {
        let width = self.terminal_width;
        if width == 0 {
            return;
        }

        let x_percent = ((x as u32) * 100 / (width as u32)) as u16;

        match self.resize_state {
            ResizeState::DraggingSessionsOutput => {
                let sessions = x_percent.clamp(10, 50);
                let remaining = 100 - sessions;
                let output = (remaining * self.panel_widths.1) / (self.panel_widths.1 + self.panel_widths.2);
                let files = remaining - output;
                self.panel_widths = (sessions, output.max(10), files.max(10));
            }
            ResizeState::DraggingOutputFiles => {
                let sessions = self.panel_widths.0;
                let output = (x_percent.saturating_sub(sessions)).clamp(10, 80 - sessions);
                let files = 100 - sessions - output;
                self.panel_widths = (sessions, output, files.max(10));
            }
            ResizeState::None => {}
        }
    }

    /// Handle keyboard/mouse input
    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(key) => self.handle_key(key),
            Event::Mouse(mouse) => self.handle_mouse(mouse),
            Event::Tick => self.on_tick(),
            Event::Resize(w, _h) => {
                self.terminal_width = w;
            }
        }
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        use crossterm::event::KeyCode;

        // Handle fuzzy finder input first
        if self.show_fuzzy_finder {
            match key.code {
                KeyCode::Esc => {
                    self.show_fuzzy_finder = false;
                    self.fuzzy_input.clear();
                }
                KeyCode::Enter => {
                    // TODO: Select fuzzy finder result
                    self.show_fuzzy_finder = false;
                    self.fuzzy_input.clear();
                }
                KeyCode::Char(c) => {
                    self.fuzzy_input.push(c);
                }
                KeyCode::Backspace => {
                    self.fuzzy_input.pop();
                }
                _ => {}
            }
            return;
        }

        // Handle help overlay
        if self.show_help {
            match key.code {
                KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => {
                    self.show_help = false;
                }
                _ => {}
            }
            return;
        }

        // Normal key handling
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Tab => self.focused_panel = self.focused_panel.next(),
            KeyCode::BackTab => self.focused_panel = self.focused_panel.prev(),
            KeyCode::Up | KeyCode::Char('k') => self.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.navigate_down(),
            KeyCode::PageUp => self.page_up(),
            KeyCode::PageDown => self.page_down(),
            KeyCode::Home => self.go_home(),
            KeyCode::End => self.go_end(),
            KeyCode::Enter => self.primary_action(),
            KeyCode::Char('?') => {
                self.show_help = true;
            }
            KeyCode::Char('/') => {
                self.show_fuzzy_finder = true;
                self.fuzzy_input.clear();
            }
            KeyCode::Char('p') if self.focused_panel == FocusedPanel::Output => {
                self.output_auto_scroll = !self.output_auto_scroll;
            }
            KeyCode::Char('.') if self.focused_panel == FocusedPanel::Files => {
                self.toggle_hidden_files();
            }
            KeyCode::Char('e') if self.focused_panel == FocusedPanel::Files => {
                self.open_selected_in_editor();
            }
            KeyCode::Char('d') if self.focused_panel == FocusedPanel::Sessions => {
                // Send interrupt to selected session
                // Note: This is async, so we just trigger it - actual send happens in tick
            }
            KeyCode::Esc => {
                // Cancel any ongoing operation or clear alert
                self.resize_state = ResizeState::None;
                self.clear_alert();
            }
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: crossterm::event::MouseEvent) {
        use crossterm::event::{MouseButton, MouseEventKind};

        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Check if clicking on tab bar (row 0)
                if mouse.row == 0 {
                    self.handle_tab_click(mouse.column);
                    return;
                }

                // Check if clicking on a panel border for resize
                let (sessions_end, output_end) = self.get_panel_borders();

                if (mouse.column as i16 - sessions_end as i16).abs() <= 1 {
                    self.resize_state = ResizeState::DraggingSessionsOutput;
                } else if (mouse.column as i16 - output_end as i16).abs() <= 1 {
                    self.resize_state = ResizeState::DraggingOutputFiles;
                } else {
                    // Click to focus panel and select item
                    self.handle_panel_click(mouse.column, mouse.row);
                }
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.resize_state != ResizeState::None {
                    self.update_resize(mouse.column);
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                self.resize_state = ResizeState::None;
            }
            MouseEventKind::ScrollUp => {
                self.scroll_panel_up(mouse.column);
            }
            MouseEventKind::ScrollDown => {
                self.scroll_panel_down(mouse.column);
            }
            _ => {}
        }
    }

    /// Handle click on the tab bar to select a session
    fn handle_tab_click(&mut self, x: u16) {
        if self.sessions.is_empty() {
            return;
        }

        // Calculate tab positions
        // Format: "[N] name " for each tab
        let mut current_x: u16 = 0;

        for (i, session) in self.sessions.iter().enumerate() {
            // Calculate this tab's width
            // "[N] " = 4 chars for single digit, 5 for double digit
            let num_width = if i + 1 >= 10 { 5 } else { 4 };

            // Parse session name like tab_bar.rs does
            let tab_text = if let Some((project, workspace)) = session.name.split_once('/') {
                format!("{} ({})", project, workspace)
            } else if let Some((project, workspace)) = session.name.rsplit_once('-') {
                if !project.is_empty() && !workspace.is_empty() && project.len() > 2 {
                    format!("{} ({})", project, workspace)
                } else {
                    session.name.clone()
                }
            } else {
                session.name.clone()
            };

            // " name " = name.len() + 2
            let tab_width = num_width + tab_text.len() as u16 + 2;

            // Add separator space
            let total_width = tab_width + 1;

            if x >= current_x && x < current_x + total_width {
                // Clicked on this tab
                if self.selected_session != i {
                    self.selected_session = i;
                    self.sync_to_session();
                }
                return;
            }

            current_x += total_width;
        }
    }

    /// Get panel border positions in terminal columns
    /// Must match the layout calculation in ui/layout.rs
    fn get_panel_borders(&self) -> (u16, u16) {
        const MIN_PANEL_WIDTH: u16 = 15;
        let width = self.terminal_width;

        // Match the actual layout calculation
        let sessions_width = ((self.panel_widths.0 as f32 / 100.0 * width as f32) as u16).max(MIN_PANEL_WIDTH);
        let output_width = ((self.panel_widths.1 as f32 / 100.0 * width as f32) as u16).max(MIN_PANEL_WIDTH);

        let sessions_end = sessions_width;
        let output_end = sessions_end + output_width;
        (sessions_end, output_end)
    }

    /// Handle click within a panel to focus and select
    fn handle_panel_click(&mut self, x: u16, y: u16) {
        let (sessions_end, output_end) = self.get_panel_borders();

        // Account for tab bar (1 line) and panel border (1 line)
        // y=0 is tab bar, y=1 is panel title/border, y>=2 is content
        let content_y = y.saturating_sub(2) as usize;

        if x < sessions_end {
            self.focused_panel = FocusedPanel::Sessions;
            // Sessions are now simple 1 row per session
            if !self.sessions.is_empty() {
                let old_selection = self.selected_session;
                let index = content_y + self.sessions_scroll;
                if index < self.sessions.len() {
                    self.selected_session = index;
                    // Sync file tree if selection changed
                    if old_selection != self.selected_session {
                        self.sync_to_session();
                    }
                }
            }
        } else if x < output_end {
            self.focused_panel = FocusedPanel::Output;
            self.output_auto_scroll = false;
        } else {
            self.focused_panel = FocusedPanel::Files;
            let index = content_y + self.files_scroll;
            if index < self.file_count {
                self.selected_file = index;
            }
        }
    }

    /// Scroll panel up based on mouse position
    fn scroll_panel_up(&mut self, x: u16) {
        let (sessions_end, output_end) = self.get_panel_borders();

        if x < sessions_end {
            let old_selection = self.selected_session;
            self.sessions_scroll = self.sessions_scroll.saturating_sub(3);
            if self.selected_session > 0 {
                self.selected_session = self.selected_session.saturating_sub(1);
            }
            if old_selection != self.selected_session {
                self.sync_to_session();
            }
        } else if x < output_end {
            self.output_auto_scroll = false;
            self.output_scroll = self.output_scroll.saturating_sub(3);
        } else {
            self.files_scroll = self.files_scroll.saturating_sub(3);
            if self.selected_file > 0 {
                self.selected_file = self.selected_file.saturating_sub(1);
            }
        }
    }

    /// Scroll panel down based on mouse position
    fn scroll_panel_down(&mut self, x: u16) {
        let (sessions_end, output_end) = self.get_panel_borders();

        if x < sessions_end {
            let old_selection = self.selected_session;
            self.sessions_scroll = (self.sessions_scroll + 3).min(self.session_count.saturating_sub(1));
            self.selected_session = (self.selected_session + 1).min(self.session_count.saturating_sub(1));
            if old_selection != self.selected_session {
                self.sync_to_session();
            }
        } else if x < output_end {
            self.output_auto_scroll = false;
            self.output_scroll = (self.output_scroll + 3).min(self.output_line_count.saturating_sub(1));
        } else {
            self.files_scroll = (self.files_scroll + 3).min(self.file_count.saturating_sub(1));
            self.selected_file = (self.selected_file + 1).min(self.file_count.saturating_sub(1));
        }
    }

    fn on_tick(&mut self) {
        // Note: Actual refresh happens in the async run loop
        // This is called synchronously and just marks that a tick occurred
    }

    /// Async tick handler - called from main loop
    pub async fn on_tick_async(&mut self) {
        // Refresh session data
        if let Err(e) = self.refresh_sessions().await {
            tracing::warn!("Failed to refresh sessions: {}", e);
        }

        // Refresh output buffer
        self.refresh_output().await;
    }

    /// Start or switch output capture for the selected session
    pub fn start_output_capture(&mut self) {
        if let Some(session) = self.selected_session_data() {
            let session_name = session.name.clone();

            // Skip if already capturing this session
            if self.captured_session.as_ref() == Some(&session_name) {
                return;
            }

            // Stop previous capture if any
            if let Some(handle) = self.capture_handle.take() {
                handle.stop();
            }

            // Clear the output buffer for fresh capture
            self.output_buffer = Arc::new(Mutex::new(RingBuffer::new(1000)));
            self.output_lines.clear();
            self.output_scroll = 0;
            self.output_line_count = 0;
            self.output_auto_scroll = false; // Start from top, not bottom

            // Start new capture
            let handle = self.output_capture.start_capture(
                session_name.clone(),
                Arc::clone(&self.output_buffer),
                500, // Poll every 500ms
            );

            self.capture_handle = Some(handle);
            self.captured_session = Some(session_name);
        }
    }

    /// Sync both file tree and output to the currently selected session
    pub fn sync_to_session(&mut self) {
        self.sync_file_tree_to_session();
        self.start_output_capture();
    }

    fn navigate_up(&mut self) {
        match self.focused_panel {
            FocusedPanel::Sessions => {
                let old_selection = self.selected_session;
                self.selected_session = self.selected_session.saturating_sub(1);
                // Adjust scroll if selection goes above visible area
                if self.selected_session < self.sessions_scroll {
                    self.sessions_scroll = self.selected_session;
                }
                // Sync file tree if selection changed
                if old_selection != self.selected_session {
                    self.sync_to_session();
                }
            }
            FocusedPanel::Output => {
                self.output_auto_scroll = false;
                self.output_scroll = self.output_scroll.saturating_sub(1);
            }
            FocusedPanel::Files => {
                self.selected_file = self.selected_file.saturating_sub(1);
                // Adjust scroll if selection goes above visible area
                if self.selected_file < self.files_scroll {
                    self.files_scroll = self.selected_file;
                }
            }
        }
    }

    fn navigate_down(&mut self) {
        match self.focused_panel {
            FocusedPanel::Sessions => {
                let old_selection = self.selected_session;
                if self.selected_session < self.session_count.saturating_sub(1) {
                    self.selected_session += 1;
                }
                // Sync file tree if selection changed
                if old_selection != self.selected_session {
                    self.sync_to_session();
                }
            }
            FocusedPanel::Output => {
                self.output_auto_scroll = false;
                if self.output_scroll < self.output_line_count.saturating_sub(1) {
                    self.output_scroll += 1;
                }
            }
            FocusedPanel::Files => {
                if self.selected_file < self.file_count.saturating_sub(1) {
                    self.selected_file += 1;
                }
            }
        }
    }

    fn page_up(&mut self) {
        let page_size = 10;
        match self.focused_panel {
            FocusedPanel::Sessions => {
                let old_selection = self.selected_session;
                self.selected_session = self.selected_session.saturating_sub(page_size);
                self.sessions_scroll = self.sessions_scroll.saturating_sub(page_size);
                if old_selection != self.selected_session {
                    self.sync_to_session();
                }
            }
            FocusedPanel::Output => {
                self.output_auto_scroll = false;
                self.output_scroll = self.output_scroll.saturating_sub(page_size);
            }
            FocusedPanel::Files => {
                self.selected_file = self.selected_file.saturating_sub(page_size);
                self.files_scroll = self.files_scroll.saturating_sub(page_size);
            }
        }
    }

    fn page_down(&mut self) {
        let page_size = 10;
        match self.focused_panel {
            FocusedPanel::Sessions => {
                let old_selection = self.selected_session;
                self.selected_session = (self.selected_session + page_size).min(self.session_count.saturating_sub(1));
                self.sessions_scroll = (self.sessions_scroll + page_size).min(self.session_count.saturating_sub(1));
                if old_selection != self.selected_session {
                    self.sync_to_session();
                }
            }
            FocusedPanel::Output => {
                self.output_auto_scroll = false;
                self.output_scroll = (self.output_scroll + page_size).min(self.output_line_count.saturating_sub(1));
            }
            FocusedPanel::Files => {
                self.selected_file = (self.selected_file + page_size).min(self.file_count.saturating_sub(1));
                self.files_scroll = (self.files_scroll + page_size).min(self.file_count.saturating_sub(1));
            }
        }
    }

    fn go_home(&mut self) {
        match self.focused_panel {
            FocusedPanel::Sessions => {
                let old_selection = self.selected_session;
                self.selected_session = 0;
                self.sessions_scroll = 0;
                if old_selection != self.selected_session {
                    self.sync_to_session();
                }
            }
            FocusedPanel::Output => {
                self.output_auto_scroll = false;
                self.output_scroll = 0;
            }
            FocusedPanel::Files => {
                self.selected_file = 0;
                self.files_scroll = 0;
            }
        }
    }

    fn go_end(&mut self) {
        match self.focused_panel {
            FocusedPanel::Sessions => {
                let old_selection = self.selected_session;
                self.selected_session = self.session_count.saturating_sub(1);
                if old_selection != self.selected_session {
                    self.sync_to_session();
                }
            }
            FocusedPanel::Output => {
                self.output_auto_scroll = true;
                self.output_scroll = self.output_line_count.saturating_sub(1);
            }
            FocusedPanel::Files => {
                self.selected_file = self.file_count.saturating_sub(1);
            }
        }
    }

    fn primary_action(&mut self) {
        match self.focused_panel {
            FocusedPanel::Sessions => {
                // TODO: Attach to selected session
            }
            FocusedPanel::Output => {
                // TODO: Send command
            }
            FocusedPanel::Files => {
                // Toggle expansion if directory, otherwise could preview file
                self.toggle_file_expand();
            }
        }
    }
}

/// Run the application
pub async fn run(config: Config) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new(config);

    // Get initial terminal size
    if let Ok((width, _height)) = crossterm::terminal::size() {
        app.terminal_width = width;
    }

    // Initial session load
    if let Err(e) = app.refresh_sessions().await {
        tracing::warn!("Failed to load initial sessions: {}", e);
    }

    // Start output capture for first session if available
    app.start_output_capture();

    // Create event handler
    let events = EventHandler::new(500); // 500ms tick rate to match output polling

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|frame| ui::draw(frame, &app))?;

        // Handle events
        let event = events.next()?;

        // Check if this is a tick event for async refresh
        let is_tick = matches!(event, Event::Tick);

        app.handle_event(event);

        // Async refresh on tick
        if is_tick {
            app.on_tick_async().await;
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Handle pending editor launch
    if let Some(file_path) = app.pending_editor_file {
        open_in_editor(&file_path)?;
    }

    Ok(())
}

/// Open a file in the external editor
fn open_in_editor(path: &std::path::Path) -> Result<()> {
    use std::process::Command;

    // Get editor from environment, with fallbacks
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            // Platform-specific fallbacks
            if cfg!(target_os = "macos") {
                "nano".to_string()
            } else if cfg!(target_os = "windows") {
                "notepad".to_string()
            } else {
                "vi".to_string()
            }
        });

    // Split editor command in case it has arguments (e.g., "code --wait")
    let mut parts = editor.split_whitespace();
    let program = parts.next().unwrap_or("vi");
    let args: Vec<&str> = parts.collect();

    let status = Command::new(program)
        .args(&args)
        .arg(path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status: {:?}", status.code());
    }

    Ok(())
}
