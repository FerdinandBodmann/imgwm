use anyhow::Result;
use tracing::info;
use calloop::EventLoop;
use std::sync::{Arc, Mutex};

mod layout;
mod input;
mod config;
mod state;

use state::WmState;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting imgwm - Wayland Tiling Window Manager");

    // Create and run the compositor
    let mut compositor = Compositor::new()?;
    compositor.run()?;

    Ok(())
}

pub struct Compositor {
    state: Arc<Mutex<WmState>>,
}

impl Compositor {
    pub fn new() -> Result<Self> {
        info!("Initializing Window Manager");

        // Initialize window manager state
        let state = Arc::new(Mutex::new(WmState::new(1920, 1080)));

        Ok(Self { state })
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Compositor initialized");
        info!("Window manager is ready to receive connections");

        let _event_loop: EventLoop<()> = EventLoop::try_new()?;
        let mut state = self.state.lock().unwrap();

        // Demo: Create some windows
        info!("\n=== Demo Mode ===");
        state.add_window("Firefox".to_string());
        state.add_window("Terminal".to_string());
        state.add_window("Text Editor".to_string());

        // Print the layout
        state.print_layout();

        // Simulate input
        info!("\nSimulating Super+M to cycle layout...");
        state.cycle_layout();
        state.print_layout();

        info!("\nDemo complete. Compositor ready for real Wayland clients.");

        Ok(())
    }
}
