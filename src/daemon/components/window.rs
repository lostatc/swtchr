use glib::Object;
use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use gtk::Application;

use super::overlay::Overlay;
use swtchr::sway;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application, title: &str) -> Self {
        Object::builder()
            .property("application", app)
            .property("title", title)
            .build()
    }

    pub fn update_windows(&self, windows: &[sway::Window]) {
        let overlay = Overlay::new();

        overlay
            .bind_property("window-id", self, "window-id")
            .sync_create()
            .build();

        overlay.update_windows(windows);

        self.set_child(Some(&overlay));
    }
}

mod imp {
    use std::cell::Cell;

    use glib::Properties;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    use swtchr::sway::SwayWindowId;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Window)]
    pub struct Window {
        #[property(get, set)]
        window_id: Cell<SwayWindowId>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "SwtchrWindow";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Window {}

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}
}
