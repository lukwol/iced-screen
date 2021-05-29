use iced::{
    button, text_input, Align, Button, Column, Command, Container, Element, Length, Text, TextInput,
};
use routing::{
    message::{Message, NavigationType},
    screen::{InitialScreen, Screen},
};

use crate::common::messages::{route::RouteMessage, screen::ScreenMessage};

use super::InputScreenMessage;

#[derive(Debug, Default)]
pub(crate) struct InputScreen {
    text_input_state: text_input::State,
    button_state: button::State,
    person_name: String,
}

impl InitialScreen<RouteMessage, ScreenMessage> for InputScreen {
    fn new(
        _flags: (),
    ) -> (
        Box<dyn Screen<RouteMessage, ScreenMessage>>,
        iced::Command<Message<RouteMessage, ScreenMessage>>,
    ) {
        (Box::new(InputScreen::default()), Command::none())
    }
}

impl Screen<RouteMessage, ScreenMessage> for InputScreen {
    fn title(&self) -> String {
        "Greeter App".to_string()
    }

    fn update(
        &mut self,
        message: ScreenMessage,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message<RouteMessage, ScreenMessage>> {
        #[allow(irrefutable_let_patterns)]
        if let ScreenMessage::InputScreen(msg) = message {
            match msg {
                InputScreenMessage::PersonNameChanged(person_name) => {
                    self.person_name = person_name
                }
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message<RouteMessage, ScreenMessage>> {
        Container::new(
            Column::new()
                .spacing(20)
                .align_items(Align::Center)
                .push(
                    Text::new("Greet Me!")
                        .height(Length::Units(20))
                        .width(Length::Units(100)),
                )
                .push(
                    TextInput::new(
                        &mut self.text_input_state,
                        "Enter your name",
                        &self.person_name,
                        |text| {
                            Message::Local(ScreenMessage::InputScreen(
                                InputScreenMessage::PersonNameChanged(text),
                            ))
                        },
                    )
                    .width(Length::Units(200))
                    .on_submit(Message::Navigate {
                        route: RouteMessage::GreetingScreenRoute {
                            person_name: self.person_name.clone(),
                        },
                        navigation_type: NavigationType::PushScreen,
                    }),
                )
                .push(
                    Button::new(&mut self.button_state, Text::new("Go!")).on_press(
                        Message::Navigate {
                            route: RouteMessage::GreetingScreenRoute {
                                person_name: self.person_name.clone(),
                            },
                            navigation_type: NavigationType::PushScreen,
                        },
                    ),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
