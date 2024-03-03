use iced::widget::{container, text};
use iced::{application, Color};

#[derive(Debug, Default)]

pub struct Theme;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::TRANSPARENT,
            text_color: Color::BLACK,
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance::default()
    }
}

impl container::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance::default().with_background(Color::TRANSPARENT)
    }
}
