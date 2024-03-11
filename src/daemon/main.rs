mod components;
mod config;
mod ipc;
mod sway;

use std::rc::Rc;

use eyre::{bail, WrapErr};
use gtk::gdk::{Display, Key, ModifierType};
use gtk::gio::{self, ActionEntry};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, CssProvider, DirectionType, EventControllerKey, Settings,
};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use signal_hook::consts::signal::SIGUSR1;
use signal_hook::iterator::Signals;

use components::Overlay;
use config::Config;
use sway::WindowSubscription;

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

type DisplayCallback = Box<dyn Fn()>;

fn register_actions(app_window: &ApplicationWindow, on_display: DisplayCallback) {
    // Make the window visible and capture keyboard events.
    let display = ActionEntry::builder("display")
        .activate(move |window: &ApplicationWindow, _, _| {
            // Check if the window is already visible first so we don't needlessly repopulate the
            // window list every time the user mashes the key.
            if !window.get_visible() {
                on_display();
                window.set_keyboard_mode(KeyboardMode::Exclusive);
                window.set_visible(true);
            }
        })
        .build();

    // Hide the window and release control of the keyboard.
    let dismiss = ActionEntry::builder("dismiss")
        .activate(|window: &ApplicationWindow, _, _| {
            window.set_keyboard_mode(KeyboardMode::None);
            window.set_visible(false);
        })
        .build();

    let focus_next = ActionEntry::builder("focus-next")
        .activate(|window: &ApplicationWindow, _, _| {
            window.child_focus(DirectionType::TabForward);
        })
        .build();

    let focus_prev = ActionEntry::builder("focus-prev")
        .activate(|window: &ApplicationWindow, _, _| {
            window.child_focus(DirectionType::TabBackward);
        })
        .build();

    let select = ActionEntry::builder("select")
        .activate(|window: &ApplicationWindow, _, _| {
            window.activate_default();
        })
        .build();

    app_window.add_action_entries([display, dismiss, focus_next, focus_prev, select]);
}

fn register_mod_release_controller(
    config: &Config,
    window: &ApplicationWindow,
) -> eyre::Result<()> {
    let dismiss_on_release = config.dismiss_on_release;
    let select_on_release = config.select_on_release;

    if !dismiss_on_release && !select_on_release {
        return Ok(());
    }

    let controller = EventControllerKey::new();

    controller.connect_key_released(clone!(@weak window => move |_, key, _, modifiers| {
        // Most keybindings are handled in the user's Sway config, calling the `swtchr` binary
        // that sends the appropriate message to the daemon over its IPC socket.
        //
        // However, we need to handle key release events in the daemon, because the way Sway
        // handles `bindsym --release` bindings wouldn't work for our use-case:
        //
        // https://github.com/swaywm/sway/pull/6920
        //
        // To simplify the config, rather than make the user specify which key release should
        // select a window and/or dismiss the overlay, we do so when both of these are true:
        //
        // 1. The user releases one of the modifier keys below.
        // 2. That modifier was the only modifier key being held.
        //
        // Why do we only perform this action when the *last* modifier is released? Imagine the
        // user is tabbing through windows via `<Super>Tab` and `<Super><Shift>Tab`. Releasing
        // `<Shift>` to switch from paging backwards to paging forwards shouldn't dismiss the
        // overlay until `<Super>` is also released.

        let is_super_released = modifiers == ModifierType::SUPER_MASK && (key == Key::Super_L || key == Key::Super_R);
        let is_shift_released = modifiers == ModifierType::SHIFT_MASK && (key == Key::Shift_L || key == Key::Shift_R);
        let is_ctrl_released = modifiers == ModifierType::CONTROL_MASK && (key == Key::Control_L || key == Key::Control_R);
        let is_alt_released = modifiers == ModifierType::ALT_MASK && (key == Key::Alt_L || key == Key::Alt_R);
        let is_hyper_released = modifiers == ModifierType::HYPER_MASK && (key == Key::Hyper_L || key == Key::Hyper_R);
        let is_meta_released = modifiers == ModifierType::META_MASK && (key == Key::Meta_L || key == Key::Meta_R);

        if is_super_released
            || is_shift_released
            || is_ctrl_released
            || is_alt_released
            || is_hyper_released
            || is_meta_released {

            if select_on_release {
                gtk::prelude::WidgetExt::activate_action(&window, "win.select", None)
                    .expect("failed to activate action to select window on key release");
            }

            if dismiss_on_release {
                gtk::prelude::WidgetExt::activate_action(&window, "win.dismiss", None)
                    .expect("failed to activate action to dismiss switcher on key release");
            }
        }
    }));

    window.add_controller(controller);

    Ok(())
}

fn build_window(config: &Config, app: &Application, subscription: Rc<WindowSubscription>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(WINDOW_TITLE)
        .build();

    set_settings(config);

    // Set this window up as an overlay via the Wayland Layer Shell protocol.
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::None);

    let overlay = Overlay::new();
    window.set_child(Some(&overlay));

    // Update the list of windows in the window switcher right before we display it.
    let on_display = Box::new(move || {
        overlay.update_windows(&subscription.get_window_list().unwrap());
    });

    register_actions(&window, on_display);
    register_mod_release_controller(config, &window).unwrap();

    // Wait for the signal to display (un-hide) the overlay.
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

fn register_keybinds(config: &Config, app: &Application) {
    // Hide the overlay.
    app.set_accels_for_action("win.dismiss", &[&config.keymap.dismiss]);

    // Focus the next app in the switcher.
    app.set_accels_for_action("win.focus-next", &[&config.keymap.next_window]);

    // Focus the previous app in the switcher.
    app.set_accels_for_action("win.focus-prev", &[&config.keymap.prev_window]);

    // Switch to the currently selected window.
    app.set_accels_for_action("win.select", &[&config.keymap.select]);
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::read()?;

    let subscription = Rc::new(
        WindowSubscription::subscribe(config.urgent_first)
            .wrap_err("failed getting Sway window subscription")?,
    );

    let app = Application::builder().application_id(APP_ID).build();

    register_keybinds(&config, &app);

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| build_window(&config, app, Rc::clone(&subscription)));

    let exit_code = app.run();

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK overlay returned a non-zero exit code.")
    }

    Ok(())
}
