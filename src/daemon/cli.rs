use clap::Parser;

/// A Gnome-style window switcher for the Sway window manager.
///
/// This is the command to start the swtchr daemon. You should run this command when you start your
/// Sway session, either via your Sway config or via a systemd service.
#[derive(Parser, Clone)]
#[command(name = "swtchrd", author, version, about)]
pub struct Cli {
    /// Override the path of the config file.
    #[arg(short, long, value_name = "PATH")]
    pub config: Option<String>,

    /// Skip checking that the daemon is running in a Sway session.
    #[arg(long)]
    pub no_check: bool,
}
