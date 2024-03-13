mod cli;

use std::os::unix::net::UnixDatagram;

use clap::Parser;
use cli::Cli;
use eyre::Context;
use swtchr_common::sock_path;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    let socket = UnixDatagram::unbound()?;
    socket
        .connect(sock_path()?)
        .wrap_err("Could not connect to swtchrd socket. Is the daemon running?")?;

    socket
        .send(args.command.command().msg())
        .wrap_err("failed sending message to the swtchrd socket")?;

    Ok(())
}
