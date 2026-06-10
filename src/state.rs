use crate::layout::{LayoutManager, Window};
use crate::input::InputHandler;
use crate::config::Config;
use std::collections::HashMap;
use tracing::info;

/// Global window manager state
pub struct WmState {
    pub layout_manager: LayoutManager,
    pub input_handler: InputHandler,
    pub config: Config,
    pub windows: HashMap<u32, WindowInfo>,
    pub next_window_id: u32,
    pub focused_window: Option<u32>,
}

/// Information about a Wayland surface/window
#[derive(Clone, Debug)]
pub struct WindowInfo {
    pub id: u32,
    pub title: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub focused: bool,
}

impl WmState {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        info!("Initializing window manager state ({}x{})", screen_width, screen_height);
        
        Self {
            layout_manager: LayoutManager::new(screen_width, screen_height),
            input_handler: InputHandler::new(),
            config: Config::default(),
            windows: HashMap::new(),
            next_window_id: 1,
            focused_window: None,
        }
    }

    pub fn add_window(&mut self, title: String) -> u32 {
        let id = self.next_window_id;
        self.next_window_id += 1;

        let window_info = WindowInfo {
            id,
            title: title.clone(),
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            focused: false,
        };

        let window = Window {
            id,
            title,
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            focused: false,
        };

        self.windows.insert(id, window_info);
        self.layout_manager.add_window(window);
        
        if self.focused_window.is_none() {
            self.focused_window = Some(id);
        }

        info!("Window added: ID={}, total windows={}", id, self.windows.len());
        id
    }

    pub fn remove_window(&mut self, id: u32) {
        self.windows.remove(&id);
        self.layout_manager.remove_window(id);

        if self.focused_window == Some(id) {
            self.focused_window = self.windows.keys().next().copied();
        }

        info!("Window removed: ID={}, remaining={}", id, self.windows.len());
    }

    pub fn set_window_title(&mut self, id: u32, title: String) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.title = title.clone();
            info!("Window title updated: ID={}, title={}", id, title);
        }
    }

    pub fn focus_window(&mut self, id: u32) {
        self.layout_manager.focus_next();
        self.focused_window = Some(id);
    }

    pub fn cycle_layout(&mut self) {
        self.layout_manager.cycle_layout();
    }

    pub fn print_layout(&self) {
        info!("=== Current Window Layout ===");
        for window in self.layout_manager.get_windows() {
            let focus_indicator = if window.focused { " [FOCUSED]" } else { "" };
            info!(
                "  {} (ID: {}) - Pos: ({}, {}), Size: {}x{}{}",
                window.title, window.id, window.x, window.y, window.width, window.height, focus_indicator
            );
        }
    }
}
