mod app_bar;
mod app_button;
mod overlay;

use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Label, Orientation, Widget};

use app_bar::AppBar;
use app_button::AppButton;

pub fn overlay() -> impl IsA<Widget> {
    let icon_bar = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(15)
        .halign(Align::Center)
        .valign(Align::Center)
        .name("overlay")
        .build();

    let window_label = Label::builder()
        .label("WezTerm - neovim")
        .justify(gtk::Justification::Center)
        .name("window-title")
        .build();

    icon_bar.append(&AppBar::new(&[
        AppButton::new(String::from("org.wezfurlong.wezterm")),
        AppButton::new(String::from("firefox")),
        AppButton::new(String::from("vlc")),
        AppButton::new(String::from("rhythmbox")),
    ]));

    icon_bar.append(&window_label);

    icon_bar
}
