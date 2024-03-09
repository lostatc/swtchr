use glib::Object;
use gtk::glib;
use gtk::glib::subclass::types::ObjectSubclassIsExt;

use super::model::Window;

glib::wrapper! {
    pub struct Overlay(ObjectSubclass<imp::Overlay>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Overlay {
    pub fn new(windows: &[Window]) -> Self {
        let obj: Self = Object::builder().build();

        obj.imp().update_windows(windows);

        obj
    }
}

mod imp {
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{Align, Label, Orientation};

    use crate::components::app_bar::AppBar;
    use crate::components::app_button::AppButton;
    use crate::components::Window;

    #[derive(Debug, Default)]
    pub struct Overlay;

    impl Overlay {
        pub fn update_windows(&self, windows: &[Window]) {
            // Remove all children.
            while let Some(child) = self.obj().last_child() {
                self.obj().remove(&child);
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

            self.obj().append(&app_bar);
            self.obj().append(&window_label);
        }
    }

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
