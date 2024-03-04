use gtk::gio::ActionEntry;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Label};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

const APP_ID: &str = "io.github.lostatc.swtchr";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_window);

    // Close window with Esc key.
    app.set_accels_for_action("win.close", &["Escape"]);

    app.run()
}

fn build_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("swtchr window switcher")
        .build();

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

    let label = Label::new(Some("My Window Switcher"));
    window.set_child(Some(&label));

    window.present();
}
