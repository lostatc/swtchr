use glib::Object;
use gtk::glib;
use gtk::prelude::*;

use crate::sway::Window;

glib::wrapper! {
    pub struct AppButton(ObjectSubclass<imp::AppButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl AppButton {
    pub fn new(window: &Window) -> Self {
        let image = window.icon_locator.icon();

        image.set_pixel_size(80);

        Object::builder()
            .property("css-classes", ["app-icon"].to_value())
            .property("child", image)
            .property("window-id", window.id)
            .property("window-title", window.title.clone())
            .build()
    }
}

mod imp {
    use std::cell::RefCell;

    use glib::Properties;
    use gtk::glib;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::AppButton)]
    pub struct AppButton {
        #[property(get, set)]
        window_id: RefCell<i64>,
        #[property(get, set)]
        window_title: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppButton {
        const NAME: &'static str = "SwtchrAppButton";
        type Type = super::AppButton;
        type ParentType = gtk::Button;
    }

    #[glib::derived_properties]
    impl ObjectImpl for AppButton {}

    impl WidgetImpl for AppButton {}

    impl ButtonImpl for AppButton {}
}
