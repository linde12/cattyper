use iced::widget::image;
use iced::{Color, Element, Length, Task as Command, Theme};
use iced_layershell::Application;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;

#[derive(Debug, Clone)]
pub enum KeyboardEventType {
    Pressed,
    Released,
}

use crate::keyboard::keyboard_event_stream;

mod keyboard;

pub fn main() -> Result<(), iced_layershell::Error> {
    Counter::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 48)),
            exclusive_zone: -1,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            margin: (8, 0, 0, 840),
            events_transparent: true,
            start_mode: StartMode::Active,
            ..Default::default()
        },
        ..Default::default()
    })
}

enum ImageState {
    BothUp,
    BothDown,
    LeftDown,
    RightDown,
}

struct Counter {
    image: ImageState,
    last_event: Option<KeyboardEventType>,
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    KeyPress(KeyboardEventType),
}

impl Application for Counter {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                image: ImageState::BothUp,
                last_event: None,
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("cattyper")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::run(keyboard_event_stream)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KeyPress(key_type) => {
                match key_type {
                    KeyboardEventType::Pressed => {
                        if let Some(KeyboardEventType::Pressed) = self.last_event {
                            self.image = ImageState::BothDown;
                        } else {
                            self.image = if rand::random() {
                                ImageState::RightDown
                            } else {
                                ImageState::LeftDown
                            }
                        }
                        self.last_event = Some(KeyboardEventType::Pressed);
                    }
                    KeyboardEventType::Released => {
                        self.image = ImageState::BothUp;
                        self.last_event = Some(KeyboardEventType::Released);
                    }
                }
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        use ImageState::*;
        let img = match self.image {
            BothUp => image("bongo-cat-both-up.png"),
            BothDown => image("bongo-cat-both-down.png"),
            LeftDown => image("bongo-cat-left-down.png"),
            RightDown => image("bongo-cat-right-down.png"),
        };
        img.width(Length::Fill).height(Length::Fill).into()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}
