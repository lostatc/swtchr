use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeymapConfig {
    #[serde(default = "defaults::dismiss_key")]
    pub dismiss: Option<String>,
    #[serde(default = "defaults::select_key")]
    pub select: Option<String>,
    #[serde(default = "defaults::peek_key")]
    pub peek: Option<String>,
    #[serde(default = "defaults::next_key")]
    pub next: Option<String>,
    #[serde(default = "defaults::prev_key")]
    pub prev: Option<String>,
    #[serde(default = "defaults::peek_next_key")]
    pub peek_next: Option<String>,
    #[serde(default = "defaults::peek_prev_key")]
    pub peek_prev: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub icon_theme: Option<String>,
    pub font: Option<String>,
    #[serde(default = "defaults::urgent_first")]
    pub urgent_first: bool,
    #[serde(default = "defaults::dismiss_on_release")]
    pub dismiss_on_release: bool,
    #[serde(default = "defaults::select_on_release")]
    pub select_on_release: bool,
    #[serde(default = "defaults::release_keys")]
    pub release_keys: Vec<String>,
    pub keymap: KeymapConfig,
}

impl Config {
    pub fn read() -> eyre::Result<Self> {
        // TODO: Parse an actual TOML file to get this config.
        Ok(Config {
            icon_theme: Some(String::from("Papirus-Dark")),
            font: Some(String::from("Fira Sans 13")),
            urgent_first: defaults::urgent_first(),
            dismiss_on_release: defaults::dismiss_on_release(),
            select_on_release: defaults::select_on_release(),
            release_keys: defaults::release_keys(),
            keymap: KeymapConfig {
                dismiss: defaults::dismiss_key(),
                select: defaults::select_key(),
                peek: defaults::peek_key(),
                next: defaults::next_key(),
                prev: defaults::prev_key(),
                peek_next: defaults::peek_next_key(),
                peek_prev: defaults::peek_prev_key(),
            },
        })
    }
}

// Any changes to the config defaults must be reflected in the example `swtchr.toml`.
mod defaults {
    pub fn urgent_first() -> bool {
        true
    }

    pub fn dismiss_on_release() -> bool {
        true
    }

    pub fn select_on_release() -> bool {
        true
    }

    pub fn release_keys() -> Vec<String> {
        vec![
            String::from("<Super>Super_L"),
            String::from("<Super><Shift>Super_L"),
        ]
    }

    pub fn dismiss_key() -> Option<String> {
        Some(String::from("Escape"))
    }

    pub fn select_key() -> Option<String> {
        None
    }

    pub fn peek_key() -> Option<String> {
        None
    }

    pub fn next_key() -> Option<String> {
        Some(String::from("<Super>Tab"))
    }

    pub fn prev_key() -> Option<String> {
        Some(String::from("<Super><Shift>Tab"))
    }

    pub fn peek_next_key() -> Option<String> {
        None
    }

    pub fn peek_prev_key() -> Option<String> {
        None
    }
}
