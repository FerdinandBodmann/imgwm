/// Input handling for keyboard and mouse
pub struct InputHandler {
    mod_key: ModifierKey,
}

#[derive(Debug, Clone, Copy)]
pub enum ModifierKey {
    Super,  // Windows key
    Alt,
    Ctrl,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            mod_key: ModifierKey::Super,
        }
    }

    /// Process keyboard shortcut
    pub fn handle_keypress(&self, modifiers: u32, keysym: u32) -> Option<Action> {
        // Simple keysym constants for common keys
        const XKB_KEY_Q: u32 = 24;
        const XKB_KEY_J: u32 = 44;
        const XKB_KEY_K: u32 = 45;
        const XKB_KEY_M: u32 = 58;
        const XKB_KEY_RETURN: u32 = 36;
        const XKB_KEY_P: u32 = 33;

        // Check if Super key (Mod4) is held
        const MOD_SUPER: u32 = 1 << 6; // Mod4 mask

        if (modifiers & MOD_SUPER) != 0 {
            match keysym {
                XKB_KEY_Q => return Some(Action::Exit),
                XKB_KEY_M => return Some(Action::CycleLayout),
                XKB_KEY_J => return Some(Action::FocusNext),
                XKB_KEY_K => return Some(Action::FocusPrev),
                XKB_KEY_RETURN => return Some(Action::SpawnTerminal),
                XKB_KEY_P => return Some(Action::SpawnMenu),
                _ => {}
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Exit,
    CycleLayout,
    FocusNext,
    FocusPrev,
    SpawnTerminal,
    SpawnMenu,
    KillWindow,
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}
