use std::env;
use std::os::unix::net::UnixDatagram;
use std::str::FromStr;

use swtchr_common::{sock_path, Command};

fn main() -> eyre::Result<()> {
    let socket = UnixDatagram::unbound()?;
    socket.connect(sock_path()?)?;

    let msg = Command::from_str(&env::args().nth(1).unwrap())?;
    socket.send(msg.msg().as_bytes())?;

    Ok(())
}
