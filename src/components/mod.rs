mod app_bar;
mod app_button;
mod overlay;

use gtk::prelude::*;
use gtk::Widget;

use crate::model::Window;

use self::overlay::Overlay;

pub fn overlay() -> impl IsA<Widget> {
    Overlay::new(&[
        Window {
            id: String::from("wezterm-id"),
            title: String::from("Wezterm - neovim"),
            icon_name: String::from("org.wezfurlong.wezterm"),
        },
        Window {
            id: String::from("firefox-id"),
            title: String::from("Firefox - GitHub"),
            icon_name: String::from("firefox"),
        },
        Window {
            id: String::from("vlc-id"),
            title: String::from("VLC - screencast.mp4"),
            icon_name: String::from("vlc"),
        },
        Window {
            id: String::from("rhythmbox-id"),
            title: String::from("Rhythmbox - Lemon Boy"),
            icon_name: String::from("rhythmbox"),
        },
    ])
}
