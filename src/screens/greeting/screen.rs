use iced::{button, Align, Button, Column, Command, Container, Element, Length, Text};
use routing::{message::Message, screen::Screen};

use crate::common::messages::{route::RouteMessage, screen::ScreenMessage};

#[derive(Debug, Default)]
pub(crate) struct GreetingScreen {
    button_state: button::State,
    person_name: String,
}

impl GreetingScreen {
    pub(crate) fn new(person_name: String) -> Self {
        GreetingScreen {
            button_state: button::State::default(),
            person_name,
        }
    }
}

impl Screen<RouteMessage, ScreenMessage> for GreetingScreen {
    fn title(&self) -> String {
        format!("Hello {}", self.person_name).to_string()
    }

    fn update(
        &mut self,
        message: ScreenMessage,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message<RouteMessage, ScreenMessage>> {
        #[allow(irrefutable_let_patterns)]
        if let ScreenMessage::GreetingScreen(_) = message {};
        Command::none()
    }

    fn view(&mut self) -> Element<Message<RouteMessage, ScreenMessage>> {
        Container::new(
            Column::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(Text::new(format!("Hello {}", self.person_name)).height(Length::Units(20)))
                .push(
                    Button::new(&mut self.button_state, Text::new("Go Back!"))
                        .on_press(Message::PopScreen),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
