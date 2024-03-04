use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &'static str = "io.github.lostatc.swtchr";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_window);

    app.run()
}

fn build_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Window switcher")
        .build();

    window.present();
}
