mod config;

use eyre::bail;
use gtk::gdk::Display;
use gtk::gio::ActionEntry;
use gtk::{glib, Align, Application, ApplicationWindow, Box, Image, Orientation, Settings};
use gtk::{prelude::*, Widget};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use config::Config;

const APP_ID: &str = "io.github.lostatc.swtchr";

fn app_icon(icon_name: &str) -> impl IsA<Widget> {
    Image::builder().icon_name(icon_name).pixel_size(80).build()
}

fn app_icon_bar() -> impl IsA<Widget> {
    let icon_bar = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(20)
        .halign(Align::Center)
        .valign(Align::Center)
        .build();

    icon_bar.append(&app_icon("firefox"));
    icon_bar.append(&app_icon("vlc"));
    icon_bar.append(&app_icon("rhythmbox"));

    icon_bar
}

fn set_settings(config: &Config) {
    let display = Display::default().expect("no default display found");
    let settings = Settings::for_display(&display);
    settings.set_gtk_icon_theme_name(config.icon_theme.as_deref());
}

fn build_window(config: &Config, app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("swtchr window switcher")
        .build();

    set_settings(config);

    // Set this window up as an overlay that captures all keyboard events via the Wayland Layer
    // Shell protocol.
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::Exclusive);

    // Close window with Esc key.
    let action_close = ActionEntry::builder("close")
        .activate(|window: &ApplicationWindow, _, _| {
            window.close();
        })
        .build();
    window.add_action_entries([action_close]);

    window.set_child(Some(&app_icon_bar()));

    window.present();
}

fn main() -> eyre::Result<()> {
    let config = Config::read()?;

    let app = Application::builder().application_id(APP_ID).build();

    // Close window with Esc key.
    app.set_accels_for_action("win.close", &[&config.keymap.dismiss]);

    app.connect_activate(move |app| build_window(&config, app));

    let exit_code = app.run();

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK app returned non-zero exit code")
    }

    Ok(())
}
