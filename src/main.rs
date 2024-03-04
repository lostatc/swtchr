use gtk::gio::ActionEntry;
use gtk::{glib, Application, ApplicationWindow, Image};
use gtk::{prelude::*, Widget};
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};

const APP_ID: &str = "io.github.lostatc.swtchr";

fn app_icon(icon_name: &str) -> impl IsA<Widget> {
    Image::builder().icon_name(icon_name).pixel_size(80).build()
}

fn app_icon_bar() -> impl IsA<Widget> {
    app_icon("firefox")
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

    window.set_child(Some(&app_icon_bar()));

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_window);

    // Close window with Esc key.
    app.set_accels_for_action("win.close", &["Escape"]);

    app.run()
}
