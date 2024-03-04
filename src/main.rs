mod config;

use eyre::bail;
use gtk::gdk::Display;
use gtk::gio::ActionEntry;
use gtk::{
    glib, Align, Application, ApplicationWindow, Box, CssProvider, Image, Label, Orientation,
    Settings,
};
use gtk::{prelude::*, Widget};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use config::Config;

const APP_ID: &str = "io.github.lostatc.swtchr";

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
    let icon_bar = Box::builder()
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

fn overlay() -> impl IsA<Widget> {
    let icon_bar = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(20)
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

fn set_settings(config: &Config) {
    let display = Display::default().expect("Could not connect to a display.");
    let settings = Settings::for_display(&display);

    settings.set_gtk_icon_theme_name(config.icon_theme.as_deref());
    settings.set_gtk_font_name(config.font.as_deref());
}

fn build_window(config: &Config, app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("swtchr")
        .build();

    set_settings(config);

    // Set this window up as an overlay that captures all keyboard events via the Wayland Layer
    // Shell protocol.
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    // Close window on keypress.
    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);

    window.set_child(Some(&overlay()));

    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() -> eyre::Result<()> {
    let config = Config::read()?;

    let app = Application::builder().application_id(APP_ID).build();

    // Close window on keypress.
    app.set_accels_for_action("win.close", &[&config.keymap.dismiss]);

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| build_window(&config, app));

    let exit_code = app.run();

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK app returned non-zero exit code")
    }

    Ok(())
}
