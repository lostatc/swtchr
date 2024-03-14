use clap::Parser;

/// A Gnome-style window switcher for the Sway window manager.
///
/// This is the client command for signaling to the swtchr daemon to open the window switcher
/// overlay.
///
/// To use swtchr, bind this command to a keyboard shortcut in your Sway config.
#[derive(Parser, Clone)]
#[command(name = "swtchr", author, version, about)]
pub struct Cli;
