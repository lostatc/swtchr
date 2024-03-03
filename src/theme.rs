use iced::widget::{container, text};
use iced::{application, color, Border, Color};

#[derive(Debug, Clone, Default)]
pub enum Style {
    #[default]
    Default,
    Switcher,
}

// Catppuccin Mocha
// https://github.com/catppuccin/catppuccin
#[derive(Debug, Copy, Clone)]
pub enum MochaColor {
    Text,
    Crust,
}

impl MochaColor {
    fn color(&self) -> Color {
        use MochaColor::*;

        match self {
            Text => color!(0xcdd6f4),
            Crust => color!(0x11111b),
        }
    }

    fn with_alpha(&self, alpha: f32) -> Color {
        let mut color = self.color();
        color.a = alpha;
        color
    }
}

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
    type Style = Style;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let mut appearance = text::Appearance::default();

        if let Style::Switcher = style {
            appearance.color = Some(MochaColor::Text.color());
        }

        appearance
    }
}

impl container::StyleSheet for Theme {
    type Style = Style;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let mut appearance = container::Appearance::default();

        if let Style::Switcher = style {
            appearance.background = Some(MochaColor::Crust.with_alpha(0.8).into());
            appearance.border = Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 20.0.into(),
            };
        }

        appearance
    }
}
