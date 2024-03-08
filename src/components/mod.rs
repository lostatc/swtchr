mod app_bar;
mod app_button;
mod model;
mod overlay;

use std::thread;
use std::time::Duration;

use gtk::prelude::*;
use gtk::Widget;

use overlay::Overlay;

use crate::config::Config;
use crate::sway::WindowSubscription;
use model::Window;

pub fn overlay(config: &Config) -> eyre::Result<impl IsA<Widget>> {
    let subscription = WindowSubscription::subscribe(config.urgent_first)?;

    thread::sleep(Duration::from_secs(5));

    let windows = subscription.get_window_list()?;

    Ok(Overlay::new(
        &windows.into_iter().map(Window::from).collect::<Vec<_>>(),
    ))

    // Overlay::new(&[
    //     Window {
    //         id: 1,
    //         title: String::from("Wezterm - neovim"),
    //         icon_name: String::from("org.wezfurlong.wezterm"),
    //     },
    //     Window {
    //         id: 2,
    //         title: String::from("Firefox - GitHub"),
    //         icon_name: String::from("firefox"),
    //     },
    //     Window {
    //         id: 3,
    //         title: String::from("VLC - screencast.mp4"),
    //         icon_name: String::from("vlc"),
    //     },
    //     Window {
    //         id: 4,
    //         title: String::from("Rhythmbox - Lemon Boy"),
    //         icon_name: String::from("rhythmbox"),
    //     },
    // ])
}
