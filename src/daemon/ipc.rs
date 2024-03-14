use std::fs;
use std::io;
use std::os::unix::net::UnixDatagram;
use std::thread;

use eyre::WrapErr;

use swtchr::ipc::{sock_path, Command};

pub fn subscribe() -> eyre::Result<async_channel::Receiver<eyre::Result<Command>>> {
    let (sender, receiver) = async_channel::unbounded::<eyre::Result<Command>>();

    let socket_path = sock_path().wrap_err("error getting IPC socket path")?;

    match fs::remove_file(&socket_path) {
        Ok(()) => {}
        Err(err) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => Err(err).wrap_err("error unlinking IPC socket")?,
    };

    let socket = UnixDatagram::bind(&socket_path).wrap_err("error binding IPC socket")?;

    thread::spawn(move || {
        let mut buf = vec![0u8; Command::BUF_LEN];

        loop {
            let send_result = match socket.recv(&mut buf) {
                Ok(num_bytes) => sender.send_blocking(Command::from_msg(&buf[..num_bytes])),
                Err(err) => sender.send_blocking(Err(err.into())),
            };

            if send_result.is_err() {
                break;
            }
        }
    });

    Ok(receiver)
}
