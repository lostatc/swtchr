mod cli;

use std::os::unix::net::UnixDatagram;

use clap::Parser;
use cli::Cli;
use eyre::Context;
use swtchr_common::{sock_path, Command};

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    // There are no CLI arguments we are interested in.
    Cli::parse();

    let socket = UnixDatagram::unbound()?;
    socket
        .connect(sock_path()?)
        .wrap_err("Could not connect to swtchrd socket. Is the daemon running?")?;

    socket
        .send(Command::Show.msg())
        .wrap_err("Failed sending a message to the swtchrd socket.")?;

    Ok(())
}
