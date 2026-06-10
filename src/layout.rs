use tracing::info;

/// Window layout strategies
#[derive(Debug, Clone, Copy)]
pub enum LayoutMode {
    Tile,
    Monocle,
    Stack,
}

/// Represents a window in the tiling layout
#[derive(Debug, Clone)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub focused: bool,
}

/// Layout manager for tiling windows
pub struct LayoutManager {
    windows: Vec<Window>,
    layout_mode: LayoutMode,
    screen_width: u32,
    screen_height: u32,
}

impl LayoutManager {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        Self {
            windows: Vec::new(),
            layout_mode: LayoutMode::Tile,
            screen_width,
            screen_height,
        }
    }

    pub fn add_window(&mut self, window: Window) {
        info!("Adding window: {} ({}x{})", window.title, window.width, window.height);
        self.windows.push(window);
        self.relayout();
    }

    pub fn remove_window(&mut self, window_id: u32) {
        self.windows.retain(|w| w.id != window_id);
        self.relayout();
    }

    pub fn relayout(&mut self) {
        match self.layout_mode {
            LayoutMode::Tile => self.tile_layout(),
            LayoutMode::Monocle => self.monocle_layout(),
            LayoutMode::Stack => self.stack_layout(),
        }
    }

    /// Tiling layout: splits screen into master + stack areas
    fn tile_layout(&mut self) {
        if self.windows.is_empty() {
            return;
        }

        let master_ratio = 0.6; // Master area takes 60% of width
        let master_width = (self.screen_width as f32 * master_ratio) as u32;
        let stack_width = self.screen_width - master_width;

        // Master window (first window)
        if !self.windows.is_empty() {
            self.windows[0].x = 0;
            self.windows[0].y = 0;
            self.windows[0].width = master_width;
            self.windows[0].height = self.screen_height;
        }

        // Stack windows (remaining windows)
        let stack_count = self.windows.len() - 1;
        if stack_count > 0 {
            let stack_height = self.screen_height / stack_count as u32;
            for (i, window) in self.windows.iter_mut().enumerate().skip(1) {
                window.x = master_width as i32;
                window.y = ((i - 1) as u32 * stack_height) as i32;
                window.width = stack_width;
                window.height = stack_height;
            }
        }
    }

    /// Monocle layout: one window at a time, fullscreen
    fn monocle_layout(&mut self) {
        for window in &mut self.windows {
            window.x = 0;
            window.y = 0;
            window.width = self.screen_width;
            window.height = self.screen_height;
        }
    }

    /// Stack layout: vertical stacking
    fn stack_layout(&mut self) {
        if self.windows.is_empty() {
            return;
        }

        let window_height = self.screen_height / self.windows.len() as u32;
        for (i, window) in self.windows.iter_mut().enumerate() {
            window.x = 0;
            window.y = (i as u32 * window_height) as i32;
            window.width = self.screen_width;
            window.height = window_height;
        }
    }

    pub fn cycle_layout(&mut self) {
        self.layout_mode = match self.layout_mode {
            LayoutMode::Tile => LayoutMode::Monocle,
            LayoutMode::Monocle => LayoutMode::Stack,
            LayoutMode::Stack => LayoutMode::Tile,
        };
        info!("Switched to layout: {:?}", self.layout_mode);
        self.relayout();
    }

    pub fn focus_next(&mut self) {
        if self.windows.is_empty() {
            return;
        }

        // Find current focused window
        let current_focused = self.windows.iter().position(|w| w.focused);
        
        // Clear all focus
        for window in &mut self.windows {
            window.focused = false;
        }

        // Set next window as focused
        let next_idx = match current_focused {
            Some(idx) => (idx + 1) % self.windows.len(),
            None => 0,
        };
        self.windows[next_idx].focused = true;
    }

    pub fn get_windows(&self) -> &[Window] {
        &self.windows
    }
}
