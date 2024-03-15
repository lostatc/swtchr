use eyre::eyre;
use gtk::gdk;
use gtk::gio::{self, DesktopAppInfo};
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
    // Return the list of string we should use to try and locate the app icon, sorted by how likely
    // they are to work.
    fn locators(&self) -> Vec<&str> {
        [
            self.app_id.as_deref(),
            self.x_window_instance.as_deref(),
            self.x_window_class.as_deref(),
            self.window_title.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn desktop_icon(&self) -> Option<gio::Icon> {
        for locator in self.locators() {
            let options = DesktopAppInfo::search(locator);

            for option_list in options {
                for desktop_file_id in option_list {
                    let maybe_icon =
                        DesktopAppInfo::new(&desktop_file_id).and_then(|app_info| app_info.icon());

                    if let Some(icon) = maybe_icon {
                        return Some(icon);
                    }
                }
            }
        }

        None
    }

    pub fn icon(&self) -> eyre::Result<gtk::Image> {
        let display = gdk::Display::default().ok_or(eyre!("Could not connect to a display."))?;
        let theme = gtk::IconTheme::for_display(&display);

        // Look for an icon in the current GTK icon theme. We should prefer use the icon in the
        // user's theme if it exists.
        for locator in self.locators() {
            if theme.has_icon(locator) {
                return Ok(gtk::Image::from_icon_name(locator));
            }
        }

        // Try to locate the app icon via its desktop entry.
        Ok(self
            .desktop_icon()
            .map(|icon| gtk::Image::from_gicon(&icon))
            // We weren't able to find an icon for the window, so fall back to the Gnome missing
            // image icon.
            .unwrap_or_else(|| gtk::Image::from_icon_name(&String::from(GTK_MISSING_IMAGE_ICON))))
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
