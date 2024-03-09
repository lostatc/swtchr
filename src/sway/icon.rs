use gtk::gio::DesktopAppInfo;
use gtk::prelude::*;
use swayipc::Node;

#[derive(Debug, Clone)]
pub struct IconLocator {
    // Only Wayland windows have an app ID.
    app_id: Option<String>,

    // Only Xwayland windows have these.
    x_window_class: Option<String>,
    x_window_instance: Option<String>,

    // Worse-case, we can fall back to the window title.
    window_title: Option<String>,
}

impl IconLocator {
    // Return the single piece of information about the window that will give us the best shot at
    // finding its icon.
    fn best_option(&self) -> Option<&str> {
        self.app_id
            .as_deref()
            .or(self.x_window_class.as_deref())
            .or(self.x_window_instance.as_deref())
            .or(self.window_title.as_deref())
    }

    fn app_info(&self) -> Option<DesktopAppInfo> {
        dbg!(self.best_option());
        let options = match self.best_option() {
            Some(best_option) => DesktopAppInfo::search(best_option),
            None => return None,
        };

        dbg!(&options);
        for option_list in options {
            for desktop_file_id in option_list {
                if let Some(app_info) = DesktopAppInfo::new(&desktop_file_id) {
                    return Some(app_info);
                }
            }
        }

        None
    }

    pub fn icon(&self) -> Option<gtk::Image> {
        match self.app_info().and_then(|app_info| app_info.icon()) {
            Some(gicon) => Some(gtk::Image::from_gicon(&gicon)),
            None => self
                .app_id
                .as_ref()
                .map(|app_id| gtk::Image::from_icon_name(app_id)),
        }
    }
}

impl From<Node> for IconLocator {
    fn from(node: Node) -> Self {
        let (x_window_class, x_window_instance) = node
            .window_properties
            .map(|props| (props.class, props.instance))
            .unzip();

        Self {
            app_id: node.app_id,
            window_title: node.name,
            x_window_class: x_window_class.flatten(),
            x_window_instance: x_window_instance.flatten(),
        }
    }
}
