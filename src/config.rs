use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeymapConfig {
    #[serde(default = "defaults::dismiss_key")]
    pub dismiss: String,
    #[serde(default = "defaults::select_key")]
    pub select: String,
    #[serde(default = "defaults::next_window_key")]
    pub next_window: String,
    #[serde(default = "defaults::prev_window_key")]
    pub prev_window: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub icon_theme: Option<String>,
    pub font: Option<String>,
    pub keymap: KeymapConfig,
}

impl Config {
    pub fn read() -> eyre::Result<Self> {
        // TODO: Parse an actual TOML file to get this config.
        Ok(Config {
            icon_theme: Some(String::from("Papirus-Dark")),
            font: Some(String::from("Fira Sans 13")),
            keymap: KeymapConfig {
                dismiss: defaults::dismiss_key(),
                select: defaults::select_key(),
                next_window: defaults::next_window_key(),
                prev_window: defaults::prev_window_key(),
            },
        })
    }
}

mod defaults {
    pub fn dismiss_key() -> String {
        String::from("Escape")
    }

    pub fn select_key() -> String {
        String::from("Return")
    }

    pub fn next_window_key() -> String {
        String::from("<Super_L>Tab")
    }

    pub fn prev_window_key() -> String {
        String::from("<Super_L><Shift_L>Tab")
    }
}
