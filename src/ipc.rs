use std::env;
use std::path::PathBuf;
use std::str::FromStr;

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
    pub fn msg(&self) -> &'static str {
        use Command::*;

        match self {
            Next => "next",
            Prev => "prev",
            PeekNext => "peek-next",
            PeekPrev => "peek-prev",
            Show => "show",
            Dismiss => "dismiss",
            Peek => "peek",
            Select => "select",
        }
    }
}

impl FromStr for Command {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        Ok(match s {
            "next" => Next,
            "prev" => Prev,
            "peek-next" => PeekNext,
            "peek-prev" => PeekPrev,
            "show" => Show,
            "dismiss" => Dismiss,
            "peek" => Peek,
            "select" => Select,
            _ => bail!("unrecognized command received over IPC socket: '{s}'"),
        })
    }
}
