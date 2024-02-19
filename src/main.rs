use iced::widget::{button, column, mouse_area, text, Column};
use iced::{
    event, executor, mouse, Alignment, Application, Command, Element, Settings, Subscription, Theme,
};

struct Counter {
    value: i32,
    button_number: mouse::Button,
}

#[derive(Debug, Clone)]
pub enum Message {
    MouseEvent(mouse::Event),
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                value: 0,
                button_number: mouse::Button::Left,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn view(&self) -> Element<Message> {
        column![
            text(self.value).size(50),
            mouse_area(button("Click_Area")),
            text(format!("{:#?}", self.button_number))
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen_with(|event, status| match event {
            event::Event::Mouse(mouse_event) => Some(Self::Message::MouseEvent(mouse_event)),
            _ => None,
        })
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::MouseEvent(mouse::Event::ButtonPressed(number)) => {
                self.button_number = number;
            }
            _ => {}
        };
        Command::none()
    }
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}
