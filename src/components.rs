use gtk::glib::IsA;
use gtk::{prelude::*, Label};
use gtk::{Align, Box as GtkBox, Image, Orientation, Widget};

fn app_icon(icon_name: &str, selected: bool) -> impl IsA<Widget> {
    let classes: &[&str] = if selected {
        &["app-icon", "selected"]
    } else {
        &["app-icon"]
    };

    Image::builder()
        .icon_name(icon_name)
        .pixel_size(80)
        .css_classes(classes)
        .build()
}

fn app_icon_bar() -> impl IsA<Widget> {
    let icon_bar = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(15)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    icon_bar.append(&app_icon("org.wezfurlong.wezterm", true));
    icon_bar.append(&app_icon("firefox", false));
    icon_bar.append(&app_icon("vlc", false));
    icon_bar.append(&app_icon("rhythmbox", false));

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
