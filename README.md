# imgwm - A Wayland Tiling Window Manager

A modern Wayland tiling window manager written in Rust, inspired by popular tiling window managers like dwm and i3.

## Features

### Current Implementation
- **Three Tiling Layouts**
  - **Tile**: Master + Stack layout (main window on left, other windows stacked on right)
  - **Monocle**: Single fullscreen window mode
  - **Stack**: Vertical stacking of all windows

- **Multi-Workspace Support**
  - 10 workspaces for organizing windows
  - Quick switching between workspaces
  - Independent layout per workspace

- **Keyboard-Driven**
  - Comprehensive key binding system
  - Customizable key mappings
  - Default bindings inspired by dwm

- **Dynamic Master Ratio**
  - Adjust master window size on-the-fly
  - Smooth workflow optimization

### Roadmap
- [ ] Full Wayland protocol implementation
- [ ] GPU rendering with wgpu
- [ ] Window decoration and borders
- [ ] Mouse support
- [ ] Configuration file support
- [ ] Multi-monitor support
- [ ] Window gaps
- [ ] Bar/panel support
- [ ] Animation support

## Architecture

```
imgwm/
├── src/
│   ├── main.rs          # Main compositor entry point
│   ├── layout.rs        # Tiling layout algorithms
│   ├── state.rs         # Global window manager state
│   ├── input.rs         # Keyboard input handling
│   └── config.rs        # Configuration management
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

### Key Components

**Compositor** (`main.rs`)
- Manages the main event loop
- Coordinates between layout manager and input handler
- Demonstrates the window manager functionality

**LayoutManager** (`layout.rs`)
- Implements the three tiling layout algorithms
- Calculates window positions and sizes
- Maintains workspace-specific layout state

**WmState** (`state.rs`)
- Manages all windows and workspaces
- Tracks focus state
- Provides high-level state mutations

**InputHandler** (`input.rs`)
- Maps keyboard events to actions
- Manages customizable key bindings
- Provides a clear action interface

## Default Key Bindings

All bindings use the **Super** (Windows) key modifier unless noted.

### Layout Management
| Binding | Action |
|---------|--------|
| `Super+M` | Cycle through layouts (Tile → Monocle → Stack) |
| `Super+Shift+N` | Reverse cycle layouts |
| `Super+H` | Increase master window width |
| `Super+L` | Decrease master window width |

### Window Management
| Binding | Action |
|---------|--------|
| `Super+J` | Focus next window |
| `Super+K` | Focus previous window |
| `Super+F` | Toggle fullscreen mode |
| `Super+Shift+D` | Kill focused window |

### Launching Applications
| Binding | Action |
|---------|--------|
| `Super+Return` | Spawn terminal |
| `Super+P` | Spawn application menu |

### System
| Binding | Action |
|---------|--------|
| `Super+Q` | Exit imgwm |

## Building

### Requirements
- Rust 1.70+
- Linux with Wayland support
- Cargo

### Build Steps

```bash
# Clone the repository
git clone https://github.com/FerdinandBodmann/imgwm.git
cd imgwm

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Run the demo
cargo run

# Run with logging
RUST_LOG=info cargo run
```

## Running

### Demo Mode (Current)
```bash
cargo run
```

The demo creates sample windows and demonstrates layout cycling and adjustments.

### As a Wayland Compositor (Future)
```bash
# Start imgwm as your Wayland compositor
imgwm &
```

Then launch Wayland-compatible applications. The compositor will automatically manage their windows.

## Configuration

Currently, configuration is hardcoded in `src/config.rs`. Future versions will support:
- JSON configuration files
- Key binding customization
- Theme/color configuration
- Layout mode preferences
- Terminal and menu command customization

Default configuration includes:
- Master ratio: 60%
- Border width: 2px
- Terminal: `alacritty`
- Menu: `wofi`

## Development

### Project Structure
The code is organized into focused modules:
- **layout.rs**: Pure layout algorithms, no dependencies on Wayland
- **state.rs**: Window and workspace state management
- **input.rs**: Input handling and key binding system
- **config.rs**: Configuration structures and loading
- **main.rs**: Compositor main loop and orchestration

### Adding New Features

**New Layout Mode:**
```rust
// In LayoutMode enum (layout.rs)
pub enum LayoutMode {
    Tile,
    Monocle,
    Stack,
    MyNewLayout,  // Add your layout
}

// Implement in LayoutManager::relayout()
LayoutMode::MyNewLayout => self.my_new_layout(),
```

**New Key Binding:**
```rust
// In InputHandler::setup_default_bindings()
const KEY_X: u32 = 48;  // Your key code
self.key_bindings.push((
    KeyBinding::new(MOD_SUPER, KEY_X),
    Action::MyNewAction
));
```

### Testing
```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test layout_tile
```

## Dependencies

Key dependencies:
- **wayland-server**: Wayland protocol implementation
- **wayland-protocols**: Wayland protocol definitions
- **calloop**: Event loop for async handling
- **tracing**: Structured logging
- **serde**: Serialization framework
- **anyhow**: Error handling

## Performance

imgwm is designed to be lightweight:
- Minimal memory footprint
- Efficient layout calculations (O(n) where n = window count)
- No unnecessary allocations during layout cycles
- Event-driven architecture

## Contributing

Contributions are welcome! Areas to help with:
1. Wayland protocol implementation
2. Rendering backend integration
3. Configuration system
4. Documentation
5. Bug fixes and testing

## Troubleshooting

### Compilation Issues
- Ensure Rust 1.70+ is installed: `rustc --version`
- Update dependencies: `cargo update`
- Clean build: `cargo clean && cargo build`

### Runtime Issues
- Enable debug logging: `RUST_LOG=debug cargo run`
- Check window positioning in demo output
- Verify layout calculations are correct

## License

imgwm is open source software. See LICENSE file for details.

## Inspiration

This project was inspired by excellent tiling window managers:
- **dwm** - Suckless dynamic window manager
- **i3** - Improved Tiling Window Manager
- **sway** - i3-compatible Wayland compositor
- **Openbox** - Box model window manager

## Roadmap

### Phase 1: Core (Current)
- [x] Layout algorithms
- [x] Window state management
- [x] Key binding system
- [x] Multi-workspace support
- [ ] Configuration system

### Phase 2: Wayland Integration
- [ ] Full protocol implementation
- [ ] Client window management
- [ ] Surface rendering
- [ ] Input device handling

### Phase 3: Features
- [ ] Window decorations
- [ ] Multi-monitor support
- [ ] Window gaps
- [ ] Animations
- [ ] Floating windows

### Phase 4: Polish
- [ ] Performance optimization
- [ ] Memory profiling
- [ ] Comprehensive testing
- [ ] User documentation

## Resources

- [Wayland Documentation](https://wayland.freedesktop.org/)
- [Smithay](https://github.com/smithay/smithay) - Wayland compositor library
- [DWM](https://dwm.suckless.org/) - Original minimalist window manager
- [i3 Manual](https://i3wm.org/docs/userguide.html)

## Contact

Questions or suggestions? Open an issue on the GitHub repository!

---

**Status**: Early Development - Not ready for daily use yet
