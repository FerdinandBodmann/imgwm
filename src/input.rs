/// Input handling for keyboard and mouse
use tracing::info;

#[derive(Debug, Clone, Copy)]
pub struct KeyBinding {
    pub modifiers: u32,
    pub key: u32,
}

impl KeyBinding {
    pub fn new(modifiers: u32, key: u32) -> Self {
        Self { modifiers, key }
    }

    pub fn is_pressed(&self, event_modifiers: u32, event_key: u32) -> bool {
        self.modifiers == event_modifiers && self.key == event_key
    }
}

pub struct InputHandler {
    pub key_bindings: Vec<(KeyBinding, Action)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Exit,
    CycleLayout,
    CycleLayoutReverse,
    FocusNext,
    FocusPrev,
    SwapMaster,
    IncMasterSize,
    DecMasterSize,
    SpawnTerminal,
    SpawnMenu,
    KillWindow,
    ToggleFullscreen,
}

impl InputHandler {
    pub fn new() -> Self {
        let mut handler = Self {
            key_bindings: Vec::new(),
        };
        handler.setup_default_bindings();
        handler
    }

    fn setup_default_bindings(&mut self) {
        const MOD_SUPER: u32 = 1 << 6; // Mod4 mask (Super/Windows key)
        const MOD_SUPER_SHIFT: u32 = MOD_SUPER | (1 << 0); // Shift

        // Layout keysyms (X11 keycodes converted to evdev)
        const KEY_Q: u32 = 24;       // q - quit
        const KEY_M: u32 = 58;       // m - cycle layout
        const KEY_N: u32 = 57;       // n - cycle layout reverse
        const KEY_J: u32 = 44;       // j - focus next
        const KEY_K: u32 = 45;       // k - focus prev
        const KEY_RETURN: u32 = 36;  // Return - spawn terminal
        const KEY_P: u32 = 33;       // p - spawn menu
        const KEY_D: u32 = 32;       // d - kill window
        const KEY_F: u32 = 41;       // f - toggle fullscreen
        const KEY_H: u32 = 43;       // h - increase master
        const KEY_L: u32 = 46;       // l - decrease master

        // Super key bindings
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_Q), Action::Exit));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_M), Action::CycleLayout));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER_SHIFT, KEY_N), Action::CycleLayoutReverse));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_J), Action::FocusNext));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_K), Action::FocusPrev));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_RETURN), Action::SpawnTerminal));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_P), Action::SpawnMenu));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER_SHIFT, KEY_D), Action::KillWindow));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_F), Action::ToggleFullscreen));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_H), Action::IncMasterSize));
        self.key_bindings.push((KeyBinding::new(MOD_SUPER, KEY_L), Action::DecMasterSize));
    }

    /// Process keyboard event and return the associated action, if any
    pub fn handle_keypress(&self, modifiers: u32, keysym: u32) -> Option<Action> {
        for (binding, action) in &self.key_bindings {
            if binding.is_pressed(modifiers, keysym) {
                info!("Key binding triggered: {:?}", action);
                return Some(*action);
            }
        }
        None
    }

    pub fn register_binding(&mut self, binding: KeyBinding, action: Action) {
        info!("Registered key binding: {:?} -> {:?}", binding, action);
        self.key_bindings.push((binding, action));
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
