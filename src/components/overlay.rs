use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::Label;

use super::model::Window;

use super::{app_bar::AppBar, app_button::AppButton};

glib::wrapper! {
    pub struct Overlay(ObjectSubclass<imp::Overlay>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Overlay {
    pub fn new(windows: &[Window]) -> Self {
        let obj: Self = Object::builder().build();

        let app_bar = AppBar::new(&windows.iter().map(AppButton::new).collect::<Vec<_>>());

        let window_label = Label::builder()
            .justify(gtk::Justification::Center)
            .name("window-title")
            .build();

        app_bar
            .bind_property("current-title", &window_label, "label")
            .sync_create()
            .build();

        obj.append(&app_bar);
        obj.append(&window_label);

        obj
    }
}

mod imp {
    use std::cell::RefCell;

    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{Align, Orientation};

    use crate::components::model::WindowList;

    #[derive(Debug, Default)]
    pub struct Overlay {
        pub windows: RefCell<WindowList>,
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
