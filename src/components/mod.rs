mod app_bar;
mod app_button;
mod model;
mod overlay;

use gtk::prelude::*;
use gtk::Widget;

use model::Window;

use self::overlay::Overlay;

pub fn overlay() -> impl IsA<Widget> {
    Overlay::new(&[
        Window {
            id: 1,
            title: String::from("Wezterm - neovim"),
            icon_name: String::from("org.wezfurlong.wezterm"),
        },
        Window {
            id: 2,
            title: String::from("Firefox - GitHub"),
            icon_name: String::from("firefox"),
        },
        Window {
            id: 3,
            title: String::from("VLC - screencast.mp4"),
            icon_name: String::from("vlc"),
        },
        Window {
            id: 4,
            title: String::from("Rhythmbox - Lemon Boy"),
            icon_name: String::from("rhythmbox"),
        },
    ])
}
