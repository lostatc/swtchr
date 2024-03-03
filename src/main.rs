use iced::widget::{
    column, container,
    image::{self, Image},
    row, text,
};
use iced::window::{self, Level};
use iced::{
    alignment, color, executor, theme, Alignment, Application, Command, Element, Length, Settings,
    Theme,
};

const ICON_HEIGHT: f32 = 120.0;

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

    fn view(&self) -> Element<'_, Self::Message> {
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
                    .height(Length::Fixed(ICON_HEIGHT))
                    .spacing(10),
                text("Window title").horizontal_alignment(alignment::Horizontal::Center)
            ]
                .align_items(Alignment::Center)
                .spacing(20)
        )
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .style(theme::Container::from(container::Appearance {
                background: Some(iced::Background::Color(color!(0x0000ff, 0.0))),
                ..Default::default()
            }))
            .into()
    }
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
    Switcher::run(Settings {
        window: window_settings(),
        ..Default::default()
    })
}
