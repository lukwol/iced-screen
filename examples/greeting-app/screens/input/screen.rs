use iced::{Command, Element};
use iced_screen::{
    message::Message,
    screen::{InitialScreen, Screen},
};

use crate::common::messages::{route::RouteMessage, screen::ScreenMessage};

use super::{state::State, view::input_view, InputScreenMessage};

#[derive(Debug, Default)]
pub(crate) struct InputScreen {
    state: State,
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
        "Greeting App".to_string()
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
                    self.state.model.person_name = person_name
                }
            }
        };
        Command::none()
    }

    fn view(&mut self) -> Element<Message<RouteMessage, ScreenMessage>> {
        input_view(&self.state.model, &mut self.state.view_state)
    }
}
