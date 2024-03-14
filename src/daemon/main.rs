mod cli;
mod components;
mod config;
mod gui;
mod ipc;

use std::rc::Rc;

use clap::Parser;
use eyre::{bail, WrapErr};
use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

use cli::Cli;
use config::Config;
use gui::{build_window, load_css};
use swtchr::session::check_is_sway_session;
use swtchr::sway::WindowSubscription;

pub const APP_ID: &str = "io.github.lostatc.swtchr";
pub const WINDOW_TITLE: &str = "swtchr";

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    gtk::init().wrap_err("Failed to initialize the GTK runtime.")?;

    let args = Cli::parse();

    let config = Config::read().wrap_err("Failed reading the swtchr.toml config file.")?;

    if !args.no_check {
        check_is_sway_session()?;
    }

    let subscription = Rc::new(
        WindowSubscription::subscribe(config.urgent_first)
            .wrap_err("Failed subscribing to Sway window focus events.")?,
    );

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| build_window(&config, app, Rc::clone(&subscription)));

    let exit_code = app.run();

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK window switcher overlay returned a non-zero exit code.")
    }

    Ok(())
}
