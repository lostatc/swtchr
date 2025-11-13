mod cli;
mod components;
mod config;
mod gui;
mod ipc;

use std::path::PathBuf;
use std::rc::Rc;

use clap::Parser;
use eyre::{bail, WrapErr};
use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

use cli::Cli;
use config::{config_file_path, Config};
use gui::{build_window, load_css};
use swtchr::sway::{check_is_sway_session, WindowSubscription};

pub const APP_ID: &str = "io.github.lostatc.swtchr";
pub const WINDOW_TITLE: &str = "swtchr";

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    gtk::init().wrap_err("Failed to initialize the GTK runtime.")?;

    let args = Cli::parse();

    let config_path = match &args.config {
        Some(path) => PathBuf::from(path),
        None => config_file_path().wrap_err("Failed getting the config file path.")?,
    };

    let config =
        Config::read(&config_path).wrap_err("Failed reading the swtchr.toml config file.")?;

    if !args.no_check {
        check_is_sway_session()?;
    }

    let subscription = Rc::new(
        WindowSubscription::subscribe(config.urgent_first,config.scratch_pad)
            .wrap_err("Failed subscribing to Sway window focus events.")?,
    );

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css().expect("Failed to load the GTK CSS."));
    app.connect_activate(move |app| build_window(&config, app, Rc::clone(&subscription)));

    // GTK attempts to do its own CLI argument parsing. We need to override that to avoid it
    // erroring on CLI args it doesn't recognize.
    let exit_code = app.run_with_args::<glib::GString>(&[]);

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK window switcher overlay returned a non-zero exit code.")
    }

    Ok(())
}
