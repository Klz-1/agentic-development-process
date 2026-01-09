//! Panel layout and arrangement
//!
//! Provides layout utilities for the three-panel TUI design with resizable panels.

#![allow(dead_code)]

use ratatui::prelude::*;

/// Minimum panel width in characters
const MIN_PANEL_WIDTH: u16 = 15;

/// Layout regions for the three-panel view
pub struct LayoutChunks {
    pub sessions: Rect,
    pub output: Rect,
    pub files: Rect,
    pub status_bar: Rect,
}

/// Stores the sizes of each panel (as percentages)
#[derive(Debug, Clone)]
pub struct PanelSizes {
    /// Sessions panel width percentage (0.0 - 1.0)
    pub sessions: f32,
    /// Output panel width percentage (0.0 - 1.0)
    pub output: f32,
    /// Files panel width percentage (0.0 - 1.0)
    pub files: f32,
    /// Whether user is dragging a divider
    pub dragging: Option<DragTarget>,
}

/// Which divider is being dragged
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragTarget {
    /// Divider between sessions and output
    SessionsOutput,
    /// Divider between output and files
    OutputFiles,
}

impl Default for PanelSizes {
    fn default() -> Self {
        Self {
            sessions: 0.20, // 20%
            output: 0.40,   // 40%
            files: 0.40,    // 40%
            dragging: None,
        }
    }
}

impl PanelSizes {
    /// Adjust panel sizes based on mouse drag position
    pub fn resize_from_drag(&mut self, x: u16, total_width: u16) {
        let Some(target) = self.dragging else { return };

        let min_ratio = MIN_PANEL_WIDTH as f32 / total_width as f32;
        let x_ratio = x as f32 / total_width as f32;

        match target {
            DragTarget::SessionsOutput => {
                // Moving the divider between sessions and output
                let new_sessions = x_ratio.clamp(min_ratio, 1.0 - 2.0 * min_ratio);
                let diff = new_sessions - self.sessions;
                self.sessions = new_sessions;
                self.output = (self.output - diff).max(min_ratio);
            }
            DragTarget::OutputFiles => {
                // Moving the divider between output and files
                let sessions_end = self.sessions;
                let new_output_end = x_ratio.clamp(
                    sessions_end + min_ratio,
                    1.0 - min_ratio,
                );
                self.output = new_output_end - sessions_end;
                self.files = 1.0 - sessions_end - self.output;
            }
        }

        // Ensure minimums and totals sum to 1.0
        self.normalize();
    }

    /// Normalize panel sizes to ensure they sum to 1.0 and respect minimums
    fn normalize(&mut self) {
        let total = self.sessions + self.output + self.files;
        if total > 0.0 {
            self.sessions /= total;
            self.output /= total;
            self.files /= total;
        }
    }

    /// Get the x-coordinate of the first divider (between sessions and output)
    pub fn divider1_x(&self, total_width: u16) -> u16 {
        (self.sessions * total_width as f32) as u16
    }

    /// Get the x-coordinate of the second divider (between output and files)
    pub fn divider2_x(&self, total_width: u16) -> u16 {
        ((self.sessions + self.output) * total_width as f32) as u16
    }

    /// Check if a click is near a divider and return which one
    pub fn divider_at(&self, x: u16, total_width: u16) -> Option<DragTarget> {
        let tolerance = 2; // pixels
        let d1 = self.divider1_x(total_width);
        let d2 = self.divider2_x(total_width);

        if x >= d1.saturating_sub(tolerance) && x <= d1 + tolerance {
            Some(DragTarget::SessionsOutput)
        } else if x >= d2.saturating_sub(tolerance) && x <= d2 + tolerance {
            Some(DragTarget::OutputFiles)
        } else {
            None
        }
    }

    /// Start dragging a divider
    pub fn start_drag(&mut self, target: DragTarget) {
        self.dragging = Some(target);
    }

    /// Stop dragging
    pub fn stop_drag(&mut self) {
        self.dragging = None;
    }
}

/// Create the three-panel layout with status bar using default sizes
pub fn create_layout_default(area: Rect) -> LayoutChunks {
    create_layout(area, &PanelSizes::default())
}

/// Create the three-panel layout with status bar using panel width percentages from App
pub fn create_layout_from_percentages(area: Rect, widths: (u16, u16, u16)) -> LayoutChunks {
    let sizes = PanelSizes {
        sessions: widths.0 as f32 / 100.0,
        output: widths.1 as f32 / 100.0,
        files: widths.2 as f32 / 100.0,
        dragging: None,
    };
    create_layout(area, &sizes)
}

/// Create the three-panel layout with status bar
pub fn create_layout(area: Rect, sizes: &PanelSizes) -> LayoutChunks {
    // Split vertically: main area + status bar
    let vertical = Layout::vertical([
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(area);

    let main_area = vertical[0];
    let status_bar = vertical[1];

    // Calculate pixel widths from percentages
    let total_width = main_area.width;
    let sessions_width = ((sizes.sessions * total_width as f32) as u16).max(MIN_PANEL_WIDTH);
    let output_width = ((sizes.output * total_width as f32) as u16).max(MIN_PANEL_WIDTH);
    let files_width = total_width.saturating_sub(sessions_width + output_width).max(MIN_PANEL_WIDTH);

    // Split main area horizontally: sessions | output | files
    let horizontal = Layout::horizontal([
        Constraint::Length(sessions_width),
        Constraint::Length(output_width),
        Constraint::Min(files_width),
    ])
    .split(main_area);

    LayoutChunks {
        sessions: horizontal[0],
        output: horizontal[1],
        files: horizontal[2],
        status_bar,
    }
}
