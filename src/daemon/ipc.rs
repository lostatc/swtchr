use std::os::unix::net::UnixDatagram;
use std::str::{self, FromStr};
use std::sync::mpsc;
use std::thread;

use swtchr::{sock_path, Command};

pub fn subscribe() -> eyre::Result<mpsc::Receiver<eyre::Result<Command>>> {
    let (sender, receiver) = mpsc::channel::<eyre::Result<Command>>();

    let socket = UnixDatagram::bind(sock_path()?)?;

    thread::spawn(move || {
        let mut buf = vec![0u8; 64];

        loop {
            let send_result = match socket.recv(&mut buf) {
                Ok(num_bytes) => sender.send(
                    str::from_utf8(&buf[..num_bytes])
                        .map_err(eyre::Report::from)
                        .and_then(Command::from_str),
                ),
                Err(err) => sender.send(Err(err.into())),
            };

            if send_result.is_err() {
                break;
            }
        }
    });

    Ok(receiver)
}
