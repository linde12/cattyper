use iced::time::{self, Duration, Instant};
use iced::widget::image;
use iced::{Color, Element, Event, Length, Task as Command, Theme, event};
use iced_layershell::Application;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;

pub fn main() -> Result<(), iced_layershell::Error> {
    Counter::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 40)),
            exclusive_zone: -1,
            anchor: Anchor::Top | Anchor::Left | Anchor::Right,
            margin: (14, 0, 0, 340),
            events_transparent: true,
            start_mode: StartMode::Active,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Counter {
    image: usize,
    last_tick: Instant,
}

#[to_layer_message]
#[derive(Debug, Clone)]
#[doc = "Some docs"]
enum Message {
    Tick,
    IcedEvent(Event),
}

impl Application for Counter {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                image: 0,
                last_tick: Instant::now(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("bongocat")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        time::every(std::time::Duration::from_millis(100)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        let now = Instant::now();

        if now.duration_since(self.last_tick).as_millis() > 170 {
            self.image = (self.image + 1) % 4;
            self.last_tick = now;
        }

        match message {
            Message::IcedEvent(event) => {
                println!("hello {event:?}");
                Command::none()
            }
            Message::Tick => Command::none(),
            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<Message> {
        let img = match self.image {
            0 => image("bongo-cat-both-up.png"),
            1 => image("bongo-cat-both-down.png"),
            2 => image("bongo-cat-left-down.png"),
            _ => image("bongo-cat-right-down.png"),
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
