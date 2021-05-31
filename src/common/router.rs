use iced::Command;
use iced_app::{message::Message, router, screen::Screen};

use crate::screens::{greeting::GreetingScreen, input::InputScreen};

use super::messages::{self, screen::ScreenMessage};

pub(crate) struct Router {}

impl router::Router<ScreenMessage> for Router {
    type RouteMessage = messages::route::RouteMessage;

    fn initial_screen(
        _flags: (),
    ) -> (
        Box<dyn Screen<Self::RouteMessage, ScreenMessage>>,
        Command<Message<Self::RouteMessage, ScreenMessage>>,
    ) {
        (Box::new(InputScreen::default()), Command::none())
    }

    fn screen(message: Self::RouteMessage) -> Box<dyn Screen<Self::RouteMessage, ScreenMessage>> {
        match message {
            messages::route::RouteMessage::GreetingScreenRoute { person_name } => {
                Box::new(GreetingScreen::new(person_name))
            }
        }
    }
}
