use anyhow::Result;
use tracing::{info, warn};

use crate::layout::{LayoutManager, Window};
use crate::input::{InputHandler, Action};
use crate::config::Config;

pub struct Compositor {
    layout_manager: LayoutManager,
    input_handler: InputHandler,
    config: Config,
    running: bool,
    window_counter: u32,
}

impl Compositor {
    pub fn new() -> Result<Self> {
        info!("Initializing Compositor");

        let config = Config::default();
        
        // Default screen dimensions (typical 1080p)
        let screen_width = 1920;
        let screen_height = 1080;

        Ok(Self {
            layout_manager: LayoutManager::new(screen_width, screen_height),
            input_handler: InputHandler::new(),
            config,
            running: false,
            window_counter: 0,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        self.running = true;
        info!("Compositor running. Press Super+Q to exit");

        // In a real implementation, this would:
        // 1. Set up Wayland socket
        // 2. Listen for client connections
        // 3. Set up an event loop
        // 4. Render with a graphics backend (wgpu, etc.)

        // For now, we'll do a simple demo
        self.demo_run()?;

        Ok(())
    }

    fn demo_run(&mut self) -> Result<()> {
        info!("Running in demo mode...");

        // Create some demo windows
        for i in 0..3 {
            self.spawn_window(format!("Window {}", i));
        }

        info!("Demo windows created. Total: {}", self.layout_manager.get_windows().len());
        
        // Print layout
        self.print_layout();

        // Simulate some keyboard input
        info!("\nSimulating keyboard shortcuts:");
        self.handle_input(0, 58); // Super+M (cycle layout)
        self.print_layout();

        self.handle_input(0, 44); // Super+J (focus next)
        info!("Focused window after Super+J");

        self.handle_input(0, 58); // Super+M (cycle layout again)
        self.print_layout();

        info!("\nCompositor demo complete");
        Ok(())
    }

    pub fn spawn_window(&mut self, title: String) {
        self.window_counter += 1;
        let window = Window {
            id: self.window_counter,
            title,
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            focused: false,
        };

        self.layout_manager.add_window(window);
    }

    pub fn handle_input(&mut self, modifiers: u32, keysym: u32) {
        if let Some(action) = self.input_handler.handle_keypress(modifiers, keysym) {
            match action {
                Action::Exit => {
                    info!("Exit command received");
                    self.running = false;
                }
                Action::CycleLayout => {
                    self.layout_manager.cycle_layout();
                }
                Action::FocusNext => {
                    self.layout_manager.focus_next();
                }
                Action::FocusPrev => {
                    // TODO: implement focus_prev
                    warn!("FocusPrev not yet implemented");
                }
                Action::SpawnTerminal => {
                    info!("Spawning terminal: {}", self.config.terminal_command);
                    self.spawn_window("Terminal".to_string());
                }
                Action::SpawnMenu => {
                    info!("Spawning menu: {}", self.config.menu_command);
                }
                Action::KillWindow => {
                    // TODO: kill focused window
                    warn!("KillWindow not yet implemented");
                }
            }
        }
    }

    fn print_layout(&self) {
        info!("=== Current Layout ===");
        for window in self.layout_manager.get_windows() {
            let focus_indicator = if window.focused { " [FOCUSED]" } else { "" };
            info!(
                "  {} (ID: {}) - Pos: ({}, {}), Size: {}x{}{}",
                window.title, window.id, window.x, window.y, window.width, window.height, focus_indicator
            );
        }
    }
}
