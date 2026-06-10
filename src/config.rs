use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub master_ratio: f32,
    pub border_width: u32,
    pub border_color: [u8; 4],
    pub background_color: [u8; 4],
    pub terminal_command: String,
    pub menu_command: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            master_ratio: 0.6,
            border_width: 2,
            border_color: [200, 200, 200, 255],      // Light gray
            background_color: [30, 30, 30, 255],     // Dark gray
            terminal_command: "alacritty".to_string(),
            menu_command: "wofi".to_string(),
        }
    }
}

impl Config {
    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
