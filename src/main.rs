use anyhow::Result;
use tracing::{error, info};

mod compositor;
mod layout;
mod input;
mod config;

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting imgwm - Wayland Tiling Window Manager");

    // Create and run the compositor
    let mut compositor = compositor::Compositor::new()?;
    compositor.run()?;

    Ok(())
}
