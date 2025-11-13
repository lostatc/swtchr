use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::path::PathBuf;

use eyre::{bail, eyre, WrapErr};
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../swtchr.toml");

fn config_dir_path() -> eyre::Result<PathBuf> {
    Ok(env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|home_dir| PathBuf::from(home_dir).join(".config")))
        .wrap_err("Could not find the swtchr config directory. It looks like both $XDG_CONFIG_HOME and $HOME are unset.")?
        .join("swtchr"))
}

pub fn config_file_path() -> eyre::Result<PathBuf> {
    Ok(config_dir_path()?.join("swtchr.toml"))
}

fn css_file_path() -> eyre::Result<PathBuf> {
    Ok(config_dir_path()?.join("style.css"))
}

pub fn user_css_override() -> eyre::Result<Option<String>> {
    let file_result = fs::File::open(
        css_file_path().wrap_err("Failed to get the path of the user's custom CSS file.")?,
    );

    match file_result {
        Ok(mut file) => {
            let mut css_contents = String::new();

            file.read_to_string(&mut css_contents)
                .wrap_err("Failed reading the contents of the user's custom CSS file.")?;

            Ok(Some(css_contents))
        }
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err).wrap_err("Failed opening the user's custom CSS file."),
    }
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
    pub scratch_pad:bool,
}

fn validate_keybind(name: &str, key: Option<&str>) -> eyre::Result<()> {
    if let Some(key) = key {
        if gtk::accelerator_parse(key).is_none() {
            bail!("Invalid keybind for `{}`: {}", name, key);
        }
    }

    Ok(())
}

impl Config {
    fn validate(&self) -> eyre::Result<()> {
        for key in &self.release_keys {
            validate_keybind("release_keys", Some(key))?
        }

        validate_keybind("dismiss", self.keymap.dismiss.as_deref())?;
        validate_keybind("select", self.keymap.select.as_deref())?;
        validate_keybind("peek", self.keymap.peek.as_deref())?;
        validate_keybind("next", self.keymap.next.as_deref())?;
        validate_keybind("prev", self.keymap.prev.as_deref())?;
        validate_keybind("peek_next", self.keymap.peek_next.as_deref())?;
        validate_keybind("peek_prev", self.keymap.peek_prev.as_deref())?;

        Ok(())
    }

    pub fn read(path: &Path) -> eyre::Result<Self> {
        // Create the parent directory of the config file if it doesn't already exist.
        fs::create_dir_all(
            path.parent()
                .ok_or(eyre!(
                    "The config file path does not have a parent directory. This is a bug."
                ))
                .wrap_err("Failed creating the parent directory for the config file.")?,
        )?;

        let config: Config = match fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(path)
        {
            // Create the config file and write the default config to it if and only if it doesn't
            // already exist.
            Ok(mut file) => {
                file.write_all(DEFAULT_CONFIG.as_bytes())
                    .wrap_err("Failed writing the default config to the config file.")?;

                toml::from_str(DEFAULT_CONFIG)
                    .wrap_err("Failed deserializing default config. This is a bug.")?
            }

            // The config file already exists. Read it.
            Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
                let mut file =
                    fs::File::open(path).wrap_err("Failed opening the config file for reading.")?;

                // A conservative estimate of the size of the buffer we'll need.
                let mut file_contents = String::with_capacity(DEFAULT_CONFIG.len() * 2);

                file.read_to_string(&mut file_contents)
                    .wrap_err("Failed reading the contents of the config file.")?;

                toml::from_str(&file_contents)
                    .wrap_err("Failed deserializing the config file. Is it valid TOML? Double-check your syntax.")?
            }
            Err(err) => {
                Err(err).wrap_err("Failed trying to check if the config file already exists.")?
            }
        };

        config
            .validate()
            .wrap_err("There was a problem with the config file.")?;

        Ok(config)
    }
}
