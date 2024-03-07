mod components;
mod config;
mod model;

use components::overlay;
use eyre::bail;
use gtk::gdk::Display;
use gtk::gio::{self, ActionEntry};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CssProvider, Settings};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use signal_hook::consts::signal::SIGUSR1;
use signal_hook::iterator::Signals;

use config::Config;

const APP_ID: &str = "io.github.lostatc.swtchr";
const WINDOW_TITLE: &str = "swtchr";

fn set_settings(config: &Config) {
    let display = Display::default().expect("Could not connect to a display.");
    let settings = Settings::for_display(&display);

    settings.set_gtk_icon_theme_name(config.icon_theme.as_deref());
    settings.set_gtk_font_name(config.font.as_deref());
}

fn wait_for_display_signal(window: &ApplicationWindow) {
    let (sender, receiver) = async_channel::bounded(1);
    let mut signals = Signals::new([SIGUSR1]).unwrap();

    gio::spawn_blocking(move || {
        for _ in &mut signals {
            sender
                .send_blocking(())
                .expect("Channel for processing unix signals is not open.");
        }
    });

    glib::spawn_future_local(clone!(@weak window => async move {
        while let Ok(()) = receiver.recv().await {
            WidgetExt::activate_action(&window, "win.display", None).unwrap();
        }
    }));
}

fn register_actions(window: &ApplicationWindow) {
    // Make the window visible and capture keyboard events.
    let action_display = ActionEntry::builder("display")
        .activate(|window: &ApplicationWindow, _, _| {
            window.set_keyboard_mode(KeyboardMode::Exclusive);
            window.set_visible(true);
        })
        .build();

    // Hide the window and release control of the keyboard.
    let action_hide = ActionEntry::builder("hide")
        .activate(|window: &ApplicationWindow, _, _| {
            window.set_keyboard_mode(KeyboardMode::None);
            window.set_visible(false);
        })
        .build();

    window.add_action_entries([action_display, action_hide]);
}

fn build_window(config: &Config, app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(WINDOW_TITLE)
        .build();

    set_settings(config);

    // Set this window up as an overlay via the Wayland Layer Shell protocol.
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::None);

    register_actions(&window);

    // Wait for the signal to display (un-hide) the overlay.
    wait_for_display_signal(&window);

    // The window is initially hidden until it receives the signal to display itself.
    window.set_child(Some(&overlay()));
    window.present();
    window.set_visible(false);
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

fn register_keybinds(config: &Config, app: &Application) {
    // Hide the overlay.
    app.set_accels_for_action("win.hide", &[&config.keymap.dismiss]);
}

fn main() -> eyre::Result<()> {
    let config = Config::read()?;

    let app = Application::builder().application_id(APP_ID).build();

    register_keybinds(&config, &app);

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| build_window(&config, app));

    let exit_code = app.run();

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK overlay returned a non-zero exit code.")
    }

    Ok(())
}
