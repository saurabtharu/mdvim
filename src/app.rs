use std::fs;
use std::path::PathBuf;

pub struct App {
    pub files: Vec<PathBuf>,
    pub selected: usize,
    pub markdown: String,
    pub show_tree: bool,
    pub scroll_offset: u16,
    pub max_scroll: u16,
    pub last_key: Option<char>,
}

impl App {
    pub fn new() -> Self {
        let files = fs::read_dir(".")
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect();

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
        }
    }

    pub fn selected_file(&self) -> Option<&PathBuf> {
        self.files.get(self.selected)
    }

    pub fn toggle_tree(&mut self) {
        self.show_tree = !self.show_tree;
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
            }
        }
        self.last_key = None;
    }

    pub fn update_max_scroll(&mut self, line_count: u16, viewport_height: u16) {
        self.max_scroll = line_count.saturating_sub(viewport_height);
    }
}
