mod config;

use eyre::bail;
use glib::clone;
use gtk::gdk::Display;
use gtk::gio::{self, ActionEntry};
use gtk::{glib, prelude::*, Widget};
use gtk::{
    Align, Application, ApplicationWindow, Box, CssProvider, Image, Label, Orientation, Settings,
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use signal_hook::consts::signal::SIGUSR1;
use signal_hook::iterator::Signals;

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

fn build_window(config: &Config, app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("swtchr")
        .build();

    set_settings(config);

    // Set this window up as an overlay via the Wayland Layer Shell protocol.
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::None);

    // Register action to hide the window and release control of the keyboard.
    let action_hide = ActionEntry::builder("hide")
        .activate(|window: &ApplicationWindow, _, _| {
            window.set_keyboard_mode(KeyboardMode::None);
            window.set_visible(false);
        })
        .build();

    // Register action to make the window visible and capture keyboard events.
    let action_display = ActionEntry::builder("display")
        .activate(|window: &ApplicationWindow, _, _| {
            window.set_keyboard_mode(KeyboardMode::Exclusive);
            window.set_visible(true);
        })
        .build();

    window.add_action_entries([action_hide, action_display]);

    window.set_child(Some(&overlay()));

    // Wait for the signal to display itself.
    wait_for_display_signal(&window);

    // The window is initially hidden until it receives the signal to display itself.
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

fn main() -> eyre::Result<()> {
    let config = Config::read()?;

    let app = Application::builder().application_id(APP_ID).build();

    // Hide window on keypress.
    app.set_accels_for_action("win.hide", &[&config.keymap.dismiss]);

    app.connect_startup(|_| load_css());

    app.connect_activate(move |app| build_window(&config, app));

    let exit_code = app.run();

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK app returned non-zero exit code")
    }

    Ok(())
}
