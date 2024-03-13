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
    Next,
    Prev,
    PeekNext,
    PeekPrev,
    Show,
    Dismiss,
    Peek,
    Select,
}

impl Command {
    pub fn msg(&self) -> &[u8] {
        use Command::*;

        match self {
            Next => b"next",
            Prev => b"prev",
            PeekNext => b"peek-next",
            PeekPrev => b"peek-prev",
            Show => b"show",
            Dismiss => b"dismiss",
            Peek => b"peek",
            Select => b"select",
        }
    }

    pub fn from_msg(msg: &[u8]) -> eyre::Result<Self> {
        use Command::*;

        Ok(match msg {
            b"next" => Next,
            b"prev" => Prev,
            b"peek-next" => PeekNext,
            b"peek-prev" => PeekPrev,
            b"show" => Show,
            b"dismiss" => Dismiss,
            b"peek" => Peek,
            b"select" => Select,
            _ => bail!("unrecognized command received over IPC socket: '{:?}'", msg),
        })
    }
}
