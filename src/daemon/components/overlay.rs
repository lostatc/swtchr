use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::Label;

use super::app_bar::AppBar;
use super::app_button::AppButton;
use crate::sway::Window;

glib::wrapper! {
    pub struct Overlay(ObjectSubclass<imp::Overlay>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Overlay {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn update_windows(&self, windows: &[Window]) {
        // Remove all children.
        while let Some(child) = self.last_child() {
            self.remove(&child);
        }

        let app_bar = AppBar::new(&windows.iter().map(AppButton::new).collect::<Vec<_>>());

        let window_label = Label::builder()
            .name("window-title")
            .justify(gtk::Justification::Center)
            .build();

        app_bar
            .bind_property("current-title", &window_label, "label")
            .sync_create()
            .build();

        self.append(&app_bar);
        self.append(&window_label);
    }
}

impl Default for Overlay {
    fn default() -> Self {
        Self::new()
    }
}

mod imp {
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{Align, Orientation};

    #[derive(Debug, Default)]
    pub struct Overlay;

    impl Overlay {}

    #[glib::object_subclass]
    impl ObjectSubclass for Overlay {
        const NAME: &'static str = "SwtchrOverlay";
        type Type = super::Overlay;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for Overlay {
        fn constructed(&self) {
            self.obj().set_orientation(Orientation::Vertical);
            self.obj().set_spacing(15);
            self.obj().set_halign(Align::Center);
            self.obj().set_valign(Align::Center);
            self.obj().set_widget_name("overlay");
        }
    }

    impl WidgetImpl for Overlay {}

    impl BoxImpl for Overlay {}
}
