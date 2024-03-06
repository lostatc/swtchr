use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::Image;

glib::wrapper! {
    pub struct AppButton(ObjectSubclass<imp::AppButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl AppButton {
    pub fn new(icon_name: String) -> Self {
        let image = Image::builder().icon_name(icon_name).pixel_size(80).build();
        Object::builder()
            .property("css-classes", ["app-icon"].to_value())
            .property("child", image)
            .build()
    }
}

mod imp {
    use gtk::glib;
    use gtk::subclass::prelude::*;

    #[derive(Debug, Default)]
    pub struct AppButton;

    #[glib::object_subclass]
    impl ObjectSubclass for AppButton {
        const NAME: &'static str = "SwtchrAppButton";
        type Type = super::AppButton;
        type ParentType = gtk::Button;
    }

    impl ObjectImpl for AppButton {}

    impl WidgetImpl for AppButton {}

    impl ButtonImpl for AppButton {}
}
