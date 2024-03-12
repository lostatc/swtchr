mod cli;

use std::os::unix::net::UnixDatagram;

use clap::Parser;
use cli::Cli;
use swtchr_common::sock_path;

fn main() -> eyre::Result<()> {
    let args = Cli::parse();

    let socket = UnixDatagram::unbound()?;
    socket.connect(sock_path()?)?;

    socket.send(args.command.command().msg().as_bytes())?;

    Ok(())
}
