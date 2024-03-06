mod app_bar;
mod app_icon;
mod overlay;

use gtk::prelude::*;
use gtk::{Align, Box as GtkBox, Label, Orientation, Widget};

use app_icon::AppButton;

fn app_icon_bar() -> impl IsA<Widget> {
    let icon_bar = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(15)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    icon_bar.append(&AppButton::new(String::from("org.wezfurlong.wezterm")));
    icon_bar.append(&AppButton::new(String::from("firefox")));
    icon_bar.append(&AppButton::new(String::from("vlc")));
    icon_bar.append(&AppButton::new(String::from("rhythmbox")));

    icon_bar
}

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

    icon_bar.append(&app_icon_bar());
    icon_bar.append(&window_label);

    icon_bar
}
