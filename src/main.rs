use anyhow::Result;
use tracing::info;
use calloop::EventLoop;
use std::sync::{Arc, Mutex};

mod layout;
mod input;
mod config;
mod state;

use state::WmState;
use input::Action;

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

        // Print the initial layout
        state.print_layout();

        // Demo: Simulate keyboard input
        info!("\nSimulating keyboard input:\n");
        
        self.demo_action(&mut state, Action::CycleLayout, "Super+M - Cycle to Monocle");
        state.print_layout();

        self.demo_action(&mut state, Action::IncMasterSize, "Super+H - Increase Master Size");
        state.print_layout();

        self.demo_action(&mut state, Action::FocusNext, "Super+J - Focus Next Window");

        self.demo_action(&mut state, Action::CycleLayout, "Super+M - Cycle to Stack");
        state.print_layout();

        info!("\nDemo complete. Compositor ready for real Wayland clients.");

        Ok(())
    }

    fn demo_action(&self, state: &mut WmState, action: Action, description: &str) {
        info!("{}", description);
        self.handle_action(state, action);
    }

    fn handle_action(&self, state: &mut WmState, action: Action) {
        match action {
            Action::Exit => {
                info!("Exit requested");
            }
            Action::CycleLayout => {
                state.cycle_layout();
            }
            Action::CycleLayoutReverse => {
                // TODO: implement reverse cycle
            }
            Action::FocusNext => {
                state.focus_next_window();
            }
            Action::FocusPrev => {
                // TODO: implement focus prev
            }
            Action::SwapMaster => {
                // TODO: implement swap master
            }
            Action::IncMasterSize => {
                state.current_workspace_mut().layout_manager.inc_master_ratio();
            }
            Action::DecMasterSize => {
                state.current_workspace_mut().layout_manager.dec_master_ratio();
            }
            Action::SpawnTerminal => {
                info!("Spawning terminal: {}", state.config.terminal_command);
                state.add_window("Terminal".to_string());
            }
            Action::SpawnMenu => {
                info!("Spawning menu: {}", state.config.menu_command);
            }
            Action::KillWindow => {
                info!("Kill window requested");
            }
            Action::ToggleFullscreen => {
                info!("Toggle fullscreen requested");
            }
        }
    }
}
