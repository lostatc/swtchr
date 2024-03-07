#[derive(Debug, Default, Clone)]
pub struct Window {
    pub id: String,
    pub title: String,
    pub icon_name: String,
}

pub type WindowList = Vec<Window>;
