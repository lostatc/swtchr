use crate::sway::{SwayNodeId, Window as SwayWindow};

#[derive(Debug, Default, Clone)]
pub struct Window {
    pub id: SwayNodeId,
    pub title: String,
    pub icon_name: String,
}

impl From<SwayWindow> for Window {
    fn from(window: SwayWindow) -> Self {
        Self {
            id: window.id,
            title: window.window_title,
            icon_name: window.app_id,
        }
    }
}

pub type WindowList = Vec<Window>;
