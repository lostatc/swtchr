mod components;
mod config;
mod ipc;

use std::rc::Rc;

use components::Window;
use eyre::{bail, WrapErr};
use gtk::gdk::Display;
use gtk::gio::ActionEntry;
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::{Application, CssProvider, DirectionType, EventControllerKey, Settings};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

use config::Config;
use swtchr::ipc::Command as SwtchrCommand;
use swtchr::sway::{self, SwayMode, WindowSubscription};

const APP_ID: &str = "io.github.lostatc.swtchr";
const WINDOW_TITLE: &str = "swtchr";

fn set_settings(config: &Config) {
    let display = Display::default().expect("Could not connect to a display.");
    let settings = Settings::for_display(&display);

    settings.set_gtk_icon_theme_name(config.icon_theme.as_deref());
    settings.set_gtk_font_name(config.font.as_deref());
}

type DisplayCallback = Box<dyn Fn()>;

fn register_actions(app_window: &Window, on_display: DisplayCallback) {
    // Make the overlay visible and capture keyboard events.
    let show = ActionEntry::builder("show")
        .activate(move |window: &Window, _, _| {
            // Check if the window is already visible first so we don't needlessly repopulate the
            // window list every time the user mashes the key.
            if !window.get_visible() {
                on_display();
                window.set_keyboard_mode(KeyboardMode::Exclusive);
                window.set_visible(true);
            }
        })
        .build();

    // Hide the overlay and release control of the keyboard.
    let dismiss = ActionEntry::builder("dismiss")
        .activate(|window: &Window, _, _| {
            window.set_keyboard_mode(KeyboardMode::None);
            window.set_visible(false);

            // Switch Sway back to the default keybind mode, releasing exclusive control over the
            // keybinds.
            sway::switch_mode(SwayMode::Default)
                .expect("failed switching Sway back to default keybind mode");
        })
        .build();

    // Switch to the selected window and hide the overlay.
    let select = ActionEntry::builder("select")
        .activate(|window: &Window, _, _| {
            sway::switch_window(window.window_id()).unwrap();
            WidgetExt::activate_action(window, "win.dismiss", None)
                .expect("failed to activate action to dismiss window");
        })
        .build();

    // Switch to the selected window without hiding the overlay.
    let peek = ActionEntry::builder("peek")
        .activate(|window: &Window, _, _| {
            sway::switch_window(window.window_id()).unwrap();
        })
        .build();

    // Select the next window in the list.
    let next = ActionEntry::builder("next")
        .activate(|window: &Window, _, _| {
            window.child_focus(DirectionType::TabForward);
        })
        .build();

    // Select the previous window in the list.
    let prev = ActionEntry::builder("prev")
        .activate(|window: &Window, _, _| {
            window.child_focus(DirectionType::TabBackward);
        })
        .build();

    // Select the next window in the list and switch to it without hiding the overlay.
    let peek_next = ActionEntry::builder("peek-next")
        .activate(|window: &Window, _, _| {
            window.child_focus(DirectionType::TabForward);
            sway::switch_window(window.window_id()).unwrap();
        })
        .build();

    // Select the previous window in the list and switch to it without hiding the overlay.
    let peek_prev = ActionEntry::builder("peek-prev")
        .activate(|window: &Window, _, _| {
            window.child_focus(DirectionType::TabBackward);
            sway::switch_window(window.window_id()).unwrap();
        })
        .build();

    app_window.add_action_entries([
        show, dismiss, select, peek, next, prev, peek_next, peek_prev,
    ]);
}

fn register_mod_release_controller(config: &Config, window: &Window) {
    let dismiss_on_release = config.dismiss_on_release;
    let select_on_release = config.select_on_release;

    let release_key = match &config.release_key {
        Some(key) => key.clone(),
        None => return,
    };

    if !dismiss_on_release && !select_on_release {
        return;
    }

    let controller = EventControllerKey::new();

    controller.connect_key_released(
        clone!(@weak window => move |_, actual_key, _, actual_modifiers| {
            let (expected_key, expected_modifiers) = gtk::accelerator_parse(&release_key)
                .expect("invalid keybind format for `release_key`");

            if actual_key == expected_key && actual_modifiers == expected_modifiers {
                if select_on_release {
                    WidgetExt::activate_action(&window, "win.select", None)
                        .expect("failed to activate action to select window on key release");
                }

                if dismiss_on_release {
                    WidgetExt::activate_action(&window, "win.dismiss", None)
                        .expect("failed to activate action to dismiss switcher on key release");
                }
            }
        }),
    );

    window.add_controller(controller);
}

fn register_ipc_command_handlers(window: &Window) -> eyre::Result<()> {
    let receiver = ipc::subscribe()?;

    glib::spawn_future_local(clone!(@weak window => async move {
        while let Ok(msg) = receiver.recv().await {
            use SwtchrCommand::*;

            match msg.expect("error receiving IPC command") {
                Show => WidgetExt::activate_action(&window, "win.show", None).map_err(eyre::Report::from),
            }.expect("error dispatching IPC command")
        }
    }));

    Ok(())
}

fn register_keybinds(config: &Config, app: &Application) {
    if let Some(key) = &config.keymap.dismiss {
        app.set_accels_for_action("win.dismiss", &[key]);
    }

    if let Some(key) = &config.keymap.select {
        app.set_accels_for_action("win.select", &[key]);
    }

    if let Some(key) = &config.keymap.peek {
        app.set_accels_for_action("win.peek", &[key]);
    }

    if let Some(key) = &config.keymap.next {
        app.set_accels_for_action("win.next", &[key]);
    }

    if let Some(key) = &config.keymap.prev {
        app.set_accels_for_action("win.prev", &[key]);
    }

    if let Some(key) = &config.keymap.peek_next {
        app.set_accels_for_action("win.peek-next", &[key]);
    }

    if let Some(key) = &config.keymap.peek_prev {
        app.set_accels_for_action("win.peek-prev", &[key]);
    }
}

fn build_window(config: &Config, app: &Application, subscription: Rc<WindowSubscription>) {
    let window = Window::new(app, WINDOW_TITLE);

    set_settings(config);

    // Set this window up as an overlay via the Wayland Layer Shell protocol.
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(KeyboardMode::None);

    // Update the list of windows in the window switcher right before we display it.
    let on_display = Box::new(clone!(@weak window => move || {
        window.update_windows(&subscription.get_window_list().unwrap());
    }));

    register_actions(&window, on_display);
    register_keybinds(config, app);
    register_mod_release_controller(config, &window);
    register_ipc_command_handlers(&window).expect("failed subscribing to IPC events");

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
    color_eyre::install()?;

    let config = Config::read()?;

    let subscription = Rc::new(
        WindowSubscription::subscribe(config.urgent_first)
            .wrap_err("failed getting Sway window subscription")?,
    );

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(move |app| build_window(&config, app, Rc::clone(&subscription)));

    let exit_code = app.run();

    if exit_code != glib::ExitCode::SUCCESS {
        bail!("GTK overlay returned a non-zero exit code.")
    }

    Ok(())
}
