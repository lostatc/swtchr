use glib::Object;
use gtk::glib;
use gtk::prelude::*;

use super::app_button::AppButton;

glib::wrapper! {
    pub struct AppBar(ObjectSubclass<imp::AppBar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl AppBar {
    pub fn new(apps: &[AppButton]) -> Self {
        let obj: AppBar = Object::builder().build();

        for app in apps {
            obj.append(app);
        }

        obj
    }
}

mod imp {
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use gtk::{Align, Orientation};

    #[derive(Debug, Default)]
    pub struct AppBar;

    #[glib::object_subclass]
    impl ObjectSubclass for AppBar {
        const NAME: &'static str = "SwtchrAppBar";
        type Type = super::AppBar;
        type ParentType = gtk::Box;
    }

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
