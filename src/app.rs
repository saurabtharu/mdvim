use std::fs;
use std::path::PathBuf;

/// Which pane currently has focus for navigation/scrolling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FocusedPane {
    FileTree,
    Preview,
}

pub struct App {
    pub files: Vec<PathBuf>,
    pub selected: usize,
    pub markdown: String,
    pub show_tree: bool,
    pub scroll_offset: u16,
    pub max_scroll: u16,
    pub last_key: Option<char>,
    /// Which pane is currently focused when the tree is visible.
    pub focused_pane: FocusedPane,
    /// Percentage of the screen width used by the file tree (10–80).
    pub tree_width_percentage: u16,
    /// Last rendered terminal width (columns) so mouse resizing can map pixels to %.
    pub last_area_width: u16,
    /// Last rendered tree width in columns.
    pub last_tree_width_px: u16,
    /// Whether the user is currently dragging the tree/preview divider.
    pub dragging_divider: bool,
    /// Last mouse click time for double-click detection (milliseconds since epoch).
    pub last_click_time: Option<u64>,
    /// Last clicked file index for double-click detection.
    pub last_clicked_file: Option<usize>,
}

impl App {
    pub fn new() -> Self {
        let mut files: Vec<PathBuf> = fs::read_dir(".")
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();
        
        // Sort files by name (case-insensitive)
        files.sort_by(|a, b| {
            let a_name = a.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
            let b_name = b.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
            a_name.cmp(&b_name)
        });

        let markdown =
            fs::read_to_string("README.md").unwrap_or_else(|_| "No README.md found".to_string());

        Self {
            files,
            selected: 0,
            markdown,
            show_tree: true,
            scroll_offset: 0,
            max_scroll: 0,
            last_key: None,
            focused_pane: FocusedPane::FileTree,
            tree_width_percentage: 20,
            last_area_width: 0,
            last_tree_width_px: 0,
            dragging_divider: false,
            last_click_time: None,
            last_clicked_file: None,
        }
    }

    pub fn selected_file(&self) -> Option<&PathBuf> {
        self.files.get(self.selected)
    }

    pub fn toggle_tree(&mut self) {
        self.show_tree = !self.show_tree;
        if !self.show_tree {
            // When the tree is hidden, always treat the preview as focused.
            self.focused_pane = FocusedPane::Preview;
        } else {
            // When showing the tree, focus it by default.
            self.focused_pane = FocusedPane::FileTree;
        }
    }

    pub fn toggle_focus(&mut self) {
        if !self.show_tree {
            // Only the preview is visible.
            self.focused_pane = FocusedPane::Preview;
            return;
        }
        self.focused_pane = match self.focused_pane {
            FocusedPane::FileTree => FocusedPane::Preview,
            FocusedPane::Preview => FocusedPane::FileTree,
        };
    }

    pub fn focus_tree(&mut self) {
        if self.show_tree {
            self.focused_pane = FocusedPane::FileTree;
        }
    }

    pub fn focus_preview(&mut self) {
        self.focused_pane = FocusedPane::Preview;
    }

    pub fn increase_tree_width(&mut self) {
        if self.show_tree {
            // Clamp to avoid too small/too large tree.
            self.tree_width_percentage = (self.tree_width_percentage + 2).min(80);
        }
    }

    pub fn decrease_tree_width(&mut self) {
        if self.show_tree {
            self.tree_width_percentage = self.tree_width_percentage.saturating_sub(2).max(10);
        }
    }

    pub fn set_tree_width_from_column(&mut self, column: u16) {
        if !self.show_tree || self.last_area_width == 0 {
            return;
        }

        let clamped_column = column.min(self.last_area_width);
        let new_pct = ((clamped_column as u32 * 100) / self.last_area_width as u32) as u16;
        self.tree_width_percentage = new_pct.clamp(10, 80);
    }

    pub fn begin_divider_drag(&mut self) {
        if self.show_tree {
            self.dragging_divider = true;
        }
    }

    pub fn end_divider_drag(&mut self) {
        self.dragging_divider = false;
    }

    pub fn next_file(&mut self) {
        if self.selected + 1 < self.files.len() {
            self.selected += 1;
        }
        self.last_key = None;
    }

    pub fn prev_file(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
        self.last_key = None;
    }

    pub fn scroll_down(&mut self, amount: u16) {
        self.scroll_offset = self
            .scroll_offset
            .saturating_add(amount)
            .min(self.max_scroll);
        self.last_key = None;
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
        self.last_key = None;
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.last_key = None;
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = self.max_scroll;
        self.last_key = None;
    }

    pub fn open_selected_file(&mut self) {
        if let Some(file) = self.selected_file() {
            if file.is_file() {
                self.markdown =
                    fs::read_to_string(file).unwrap_or_else(|_| "Unable to read file".to_string());
                self.scroll_offset = 0;
                // After opening a file, shift focus to the preview.
                self.focus_preview();
            }
        }
        self.last_key = None;
    }

    pub fn update_max_scroll(&mut self, line_count: u16, viewport_height: u16) {
        self.max_scroll = line_count.saturating_sub(viewport_height);
    }

    /// Select a file by index (for mouse clicks)
    pub fn select_file_by_index(&mut self, index: usize) {
        if index < self.files.len() {
            self.selected = index;
            self.last_key = None;
        }
    }

    /// Get the file index at a given row position in the tree view
    pub fn get_file_index_at_row(&self, row: u16, tree_start_row: u16) -> Option<usize> {
        if row < tree_start_row {
            return None;
        }
        let index = (row - tree_start_row) as usize;
        if index < self.files.len() {
            Some(index)
        } else {
            None
        }
    }

    /// Handle mouse click on file tree
    pub fn handle_tree_click(&mut self, row: u16, tree_start_row: u16, current_time_ms: u64) -> bool {
        // Focus tree view
        self.focus_tree();
        
        // Check if clicking on a file
        if let Some(index) = self.get_file_index_at_row(row, tree_start_row) {
            // Check for double-click (within 500ms and same file)
            if let Some(last_time) = self.last_click_time {
                if let Some(last_file) = self.last_clicked_file {
                    if last_file == index && current_time_ms.saturating_sub(last_time) < 500 {
                        // Double-click detected
                        self.select_file_by_index(index);
                        self.open_selected_file();
                        self.last_click_time = None;
                        self.last_clicked_file = None;
                        return true; // File was opened
                    }
                }
            }
            
            // Single click - select file
            self.select_file_by_index(index);
            self.last_click_time = Some(current_time_ms);
            self.last_clicked_file = Some(index);
        }
        false // File was not opened
    }

    /// Handle mouse click on preview pane
    pub fn handle_preview_click(&mut self) {
        self.focus_preview();
        // Reset double-click tracking when clicking preview
        self.last_click_time = None;
        self.last_clicked_file = None;
    }
}
