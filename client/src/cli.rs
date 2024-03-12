use clap::{Parser, Subcommand};

use swtchr_common::Command as SwtchrCommand;

/// A Gnome-style window switcher for the Sway window manager.
///
/// This is the client command for controlling the swtchr daemon. The daemon subscribes to Sway
/// window focus events to sort the windows in the switcher from most recently accessed to least
/// recently accessed.
///
/// To use swtchr, bind these commands to keyboard shortcuts in your Sway config.
#[derive(Parser, Clone)]
#[command(name = "swtchr", author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommands,
}

#[derive(Subcommand, Clone)]
pub enum CliCommands {
    /// Open the window switcher and select the next window in the list.
    Next,

    /// Open the window switcher and select the previous window in the list.
    Prev,

    /// Like `next`, but focuses that window without closing the window switcher.
    PeekNext,

    /// Like `prev`, but focuses that window without closing the window switcher.
    PeekPrev,

    /// Open the window switcher.
    Show,

    /// Close the window switcher without switching window focus.
    Dismiss,

    /// Focus the currently selected window in the window switcher without closing the switcher.
    Peek,

    /// Focus the currently selected window in the window switcher and close the switcher.
    Select,
}

impl CliCommands {
    pub fn command(&self) -> SwtchrCommand {
        use CliCommands::*;

        match self {
            Next => SwtchrCommand::Next,
            Prev => SwtchrCommand::Prev,
            PeekNext => SwtchrCommand::PeekNext,
            PeekPrev => SwtchrCommand::PeekPrev,
            Show => SwtchrCommand::Show,
            Dismiss => SwtchrCommand::Dismiss,
            Peek => SwtchrCommand::Peek,
            Select => SwtchrCommand::Select,
        }
    }
}
