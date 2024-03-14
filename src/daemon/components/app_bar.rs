use glib::Object;
use gtk::glib::{self, clone};
use gtk::prelude::*;

use super::app_button::AppButton;

glib::wrapper! {
    pub struct AppBar(ObjectSubclass<imp::AppBar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AppBar {
    pub fn new(app_buttons: &[AppButton]) -> Self {
        let obj: Self = Object::builder().build();

        for button in app_buttons.iter() {
            obj.append(button);
        }

        // Initially focus the previous window instead of the current app so that the first call to
        // `swtchr next` switches to the previous window.
        if let Some(prev_app) = app_buttons.get(1) {
            obj.set_focus_child(Some(prev_app));
        }

        for app_button in app_buttons.iter() {
            app_button.connect_has_focus_notify(clone!(@weak obj => move |button| {
                obj.set_current_title(button.window_title());
                obj.set_window_id(button.window_id());
            }));
        }

        obj
    }
}

mod imp {
    use std::cell::Cell;
    use std::cell::RefCell;

    use glib::Properties;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{Align, Orientation};

    use crate::sway::SwayWindowId;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::AppBar)]
    pub struct AppBar {
        #[property(get, set)]
        current_title: RefCell<String>,
        #[property(get, set)]
        window_id: Cell<SwayWindowId>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppBar {
        const NAME: &'static str = "SwtchrAppBar";
        type Type = super::AppBar;
        type ParentType = gtk::Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for AppBar {
        fn constructed(&self) {
            self.obj().set_orientation(Orientation::Horizontal);
            self.obj().set_spacing(15);
            self.obj().set_halign(Align::Center);
            self.obj().set_valign(Align::Center);
        }
    }

    impl WidgetImpl for AppBar {}

    impl BoxImpl for AppBar {}
}
