use gtk::gio::DesktopAppInfo;
use gtk::prelude::*;
use swayipc::Node;

// The name of the standard icon used by Gnome when another icon could not be loaded.
const GTK_MISSING_IMAGE_ICON: &str = "image-missing";

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
        let options = match self.best_option() {
            Some(best_option) => DesktopAppInfo::search(best_option),
            None => return None,
        };

        for option_list in options {
            for desktop_file_id in option_list {
                if let Some(app_info) = DesktopAppInfo::new(&desktop_file_id) {
                    return Some(app_info);
                }
            }
        }

        None
    }

    pub fn icon(&self) -> gtk::Image {
        match self.app_info().and_then(|app_info| app_info.icon()) {
            // Try to locate the app icon via its desktop entry.
            Some(gicon) => gtk::Image::from_gicon(&gicon),
            // Look for an icon in the current GTK theme for the window's Wayland app ID.
            None => gtk::Image::from_icon_name(
                self.app_id
                    .as_ref()
                    // We weren't able to find an icon for the window, so fall back to the Gnome
                    // missing image icon.
                    .unwrap_or(&String::from(GTK_MISSING_IMAGE_ICON)),
            ),
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
