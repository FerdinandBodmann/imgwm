use crate::layout::{LayoutManager, Window};
use crate::input::{InputHandler, Action};
use crate::config::Config;
use std::collections::HashMap;
use tracing::info;

/// A workspace contains windows and a layout
#[derive(Debug, Clone)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub layout_manager: LayoutManager,
    pub focused_window: Option<u32>,
}

impl Workspace {
    pub fn new(id: u32, name: String, screen_width: u32, screen_height: u32) -> Self {
        Self {
            id,
            name,
            layout_manager: LayoutManager::new(screen_width, screen_height),
            focused_window: None,
        }
    }
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

/// Global window manager state
pub struct WmState {
    pub input_handler: InputHandler,
    pub config: Config,
    pub workspaces: Vec<Workspace>,
    pub active_workspace: usize,
    pub next_window_id: u32,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl WmState {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        info!("Initializing window manager state ({}x{})", screen_width, screen_height);
        
        let mut workspaces = Vec::new();
        
        // Create 10 default workspaces (like i3/dwm)
        for i in 1..=10 {
            let ws = Workspace::new(
                i,
                format!("Workspace {}", i),
                screen_width,
                screen_height,
            );
            workspaces.push(ws);
        }
        
        Self {
            input_handler: InputHandler::new(),
            config: Config::default(),
            workspaces,
            active_workspace: 0,
            next_window_id: 1,
            screen_width,
            screen_height,
        }
    }

    pub fn current_workspace(&self) -> &Workspace {
        &self.workspaces[self.active_workspace]
    }

    pub fn current_workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active_workspace]
    }

    pub fn add_window(&mut self, title: String) -> u32 {
        let id = self.next_window_id;
        self.next_window_id += 1;

        let window = Window {
            id,
            title: title.clone(),
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            focused: false,
        };

        let ws = self.current_workspace_mut();
        ws.layout_manager.add_window(window);
        ws.focused_window = Some(id);

        info!("Window added to workspace {}: ID={}, title={}", 
              self.active_workspace + 1, id, title);
        id
    }

    pub fn remove_window(&mut self, id: u32) {
        let ws = self.current_workspace_mut();
        ws.layout_manager.remove_window(id);

        if ws.focused_window == Some(id) {
            ws.focused_window = None;
        }

        info!("Window removed from workspace {}: ID={}", self.active_workspace + 1, id);
    }

    pub fn set_window_title(&mut self, id: u32, title: String) {
        info!("Window title updated: ID={}, title={}", id, title);
    }

    pub fn focus_next_window(&mut self) {
        let ws = self.current_workspace_mut();
        ws.layout_manager.focus_next();
        info!("Focused next window in workspace {}", self.active_workspace + 1);
    }

    pub fn cycle_layout(&mut self) {
        let ws = self.current_workspace_mut();
        ws.layout_manager.cycle_layout();
    }

    pub fn switch_workspace(&mut self, workspace_id: usize) {
        if workspace_id < self.workspaces.len() {
            self.active_workspace = workspace_id;
            info!("Switched to workspace {}", workspace_id + 1);
        }
    }

    pub fn print_layout(&self) {
        let ws = self.current_workspace();
        info!("=== Workspace {} - Current Layout ===", ws.id);
        for window in ws.layout_manager.get_windows() {
            let focus_indicator = if window.focused { " [FOCUSED]" } else { "" };
            info!(
                "  {} (ID: {}) - Pos: ({}, {}), Size: {}x{}{}",
                window.title, window.id, window.x, window.y, window.width, window.height, focus_indicator
            );
        }
    }
}
