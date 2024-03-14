use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use eyre::{eyre, WrapErr};
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../swtchr.toml");

fn config_file_path() -> eyre::Result<PathBuf> {
    let xdg_config_dir = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|home_dir| PathBuf::from(home_dir).join(".config")))
        .wrap_err("Could not find the swtchr config directory. It looks like both $XDG_CONFIG_HOME and $HOME are unset.")?;

    Ok(xdg_config_dir.join("swtchr").join("swtchr.toml"))
}

#[derive(Debug, Deserialize)]
pub struct KeymapConfig {
    pub dismiss: Option<String>,
    pub select: Option<String>,
    pub peek: Option<String>,
    pub next: Option<String>,
    pub prev: Option<String>,
    pub peek_next: Option<String>,
    pub peek_prev: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub icon_theme: Option<String>,
    pub font: Option<String>,
    pub urgent_first: bool,
    pub dismiss_on_release: bool,
    pub select_on_release: bool,
    pub release_keys: Vec<String>,
    pub keymap: KeymapConfig,
}

impl Config {
    pub fn read() -> eyre::Result<Self> {
        let config_path = config_file_path().wrap_err("Failed getting the config file path.")?;

        // Create the parent directory of the config file if it doesn't already exist.
        fs::create_dir_all(
            config_path
                .parent()
                .ok_or(eyre!(
                    "The config file path does not have a parent directory. This is a bug."
                ))
                .wrap_err("Failed creating the parent directory for the config file.")?,
        )?;

        match fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&config_path)
        {
            // Create the config file and write the default config to it if and only if it doesn't
            // already exist.
            Ok(mut file) => {
                file.write_all(DEFAULT_CONFIG.as_bytes())
                    .wrap_err("Failed writing the default config to the config file.")?;

                toml::from_str(DEFAULT_CONFIG)
                    .wrap_err("Failed deserializing default config. This is a bug.")
            }

            // The config file already exists. Read it.
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                let mut file = fs::File::open(&config_path)
                    .wrap_err("Failed opening the config file for reading.")?;

                // A conservative estimate of the size of the buffer we'll need.
                let mut file_contents = String::with_capacity(DEFAULT_CONFIG.len() * 2);

                file.read_to_string(&mut file_contents)
                    .wrap_err("Failed reading the contents of the config file.")?;

                toml::from_str(&file_contents)
                    .wrap_err("Failed deserializing the config file. Is it valid TOML? Double-check your syntax.")
            }
            Err(err) => {
                Err(err).wrap_err("Failed trying to check if the config file already exists.")?
            }
        }
    }
}
