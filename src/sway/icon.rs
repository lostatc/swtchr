use std::path::PathBuf;

use gtk::gio::{DesktopAppInfo, FileIcon, ThemedIcon};
use gtk::glib::object::Cast;
use gtk::prelude::*;
use swayipc::Node;

pub struct IconLocator {
    // Only Wayland windows have an app ID.
    app_id: Option<String>,

    // Only Xwayland windows have these.
    x_window_class: Option<String>,
    x_window_instance: Option<String>,

    // Worse-case, we can fall back to the window title.
    window_title: Option<String>,
}

pub enum Icon {
    // An icon in the user's GTK theme that we can load by name.
    Theme { name: String },

    // An icon located at a path in the filesystem.
    File { path: PathBuf },
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

    pub fn icon(&self) -> Option<Icon> {
        let gio_icon = self.app_info()?.icon()?;

        if let Some(themed_icon) = gio_icon.downcast_ref::<ThemedIcon>() {
            if let &[icon_name, ..] = &themed_icon.names().as_slice() {
                return Some(Icon::Theme {
                    name: icon_name.to_string(),
                });
            }
        }

        if let Some(file_icon) = gio_icon.downcast_ref::<FileIcon>() {
            if let Some(path) = file_icon.file().path() {
                return Some(Icon::File { path });
            }
        }

        None
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
