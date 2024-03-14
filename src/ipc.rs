use std::env;
use std::path::PathBuf;

use eyre::bail;

const SOCK_NAME: &str = "swtchrd.sock";

pub fn sock_path() -> eyre::Result<PathBuf> {
    match env::var("XDG_RUNTIME_DIR")?.trim() {
        "" => {
            let uid = nix::unistd::getuid();
            Ok(PathBuf::from(format!("/run/user/{uid}/{SOCK_NAME}")))
        }
        path => Ok([path, SOCK_NAME].iter().collect()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Command {
    Show,
}

impl Command {
    pub const BUF_LEN: usize = 16;

    pub fn msg(&self) -> &[u8] {
        use Command::*;

        match self {
            Show => b"show",
        }
    }

    pub fn from_msg(msg: &[u8]) -> eyre::Result<Self> {
        use Command::*;

        Ok(match msg {
            b"show" => Show,
            _ => bail!(
                "Unrecognized command received over swtchrd IPC socket: '{:?}'.",
                msg
            ),
        })
    }
}
