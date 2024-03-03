mod theme;

use std::env;

use iced::application::StyleSheet;
use iced::widget::{
    column, container,
    image::{self, Image},
    row, text,
};
use iced::window::{self, Level};
use iced::{alignment, executor, Alignment, Application, Command, Element, Length, Settings};

use theme::{Style, Theme};

#[derive(Debug)]
struct Switcher {
    selected_index: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Switch { index: u32 },
    Select,
    Exit,
}

impl Application for Switcher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (Self { selected_index: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("Switch windows")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Switch { index } => {
                self.selected_index = index;
                Command::none()
            }
            Message::Select => todo!(),
            Message::Exit => window::close(window::Id::MAIN),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme> {
        container(
            container(
                column![
                    row![
                        Image::<image::Handle>::new(
                            "/home/wren/.local/share/icons/hicolor/512x512/apps/brave-kjbdgfilnfhdoflbpgamdcdgpehopbep-Default.png"
                        ),
                        Image::<image::Handle>::new(
                            "/home/wren/.local/share/icons/hicolor/512x512/apps/brave-hpfldicfbfomlpcikngkocigghgafkph-Default.png"
                        ),
                        Image::<image::Handle>::new(
                            "/home/wren/.local/share/icons/hicolor/512x512/apps/brave-jnpecgipniidlgicjocehkhajgdnjekh-Default.png"
                        )
                    ]
                        .align_items(Alignment::Center)
                        .height(Length::Fixed(80.0))
                        .spacing(20),
                    text("Window title").style(Style::Switcher).horizontal_alignment(alignment::Horizontal::Center)
                ]
                    .align_items(Alignment::Center)
                    .spacing(30)
            )
                .style(Style::Switcher)
                .height(Length::Shrink)
                .width(Length::Shrink)
                .padding([25, 35])
        )
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn style(&self) -> <Self::Theme as StyleSheet>::Style {}
}

fn window_settings() -> window::Settings {
    window::Settings {
        decorations: false,
        level: Level::AlwaysOnTop,
        position: window::Position::Centered,
        resizable: false,
        transparent: true,
        ..Default::default()
    }
}

fn main() -> iced::Result {
    // Force the `gl` backend because window transparency doesn't work on the default wgpu backend.
    //
    // https://github.com/iced-rs/iced/issues/596
    env::set_var("WGPU_BACKEND", "gl");

    Switcher::run(Settings {
        window: window_settings(),
        ..Default::default()
    })
}
