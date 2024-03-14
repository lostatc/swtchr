mod cli;

use std::os::unix::net::UnixDatagram;

use clap::Parser;
use eyre::Context;
use swtchr::ipc::{sock_path, Command};
use swtchr::session::check_is_sway_session;
use swtchr::sway;

use cli::Cli;

fn send_msg() -> eyre::Result<()> {
    let socket = UnixDatagram::unbound()?;
    socket
        .connect(sock_path()?)
        .wrap_err("Could not connect to swtchrd socket. Is the daemon running?")?;

    socket
        .send(Command::Show.msg())
        .wrap_err("Failed sending a message to the swtchrd socket.")?;

    Ok(())
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    // There are no CLI arguments we are interested in.
    Cli::parse();

    check_is_sway_session()?;

    if let Err(err) = send_msg() {
        // We weren't able to message the swtchrd socket to open the window switcher, so it can't
        // switch the Sway binding mode back to `default` for us. To avoid locking the user into
        // the `swtchr` binding mode, we should change the binding mode back to `default` here.
        sway::switch_mode(sway::SwayMode::Default)?;

        return Err(err);
    }

    Ok(())
}
