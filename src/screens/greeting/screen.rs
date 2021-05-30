use iced::{button, Command, Element};
use routing::{message::Message, screen::Screen};

use crate::common::messages::{route::RouteMessage, screen::ScreenMessage};

use super::{
    state::{Model, State, ViewState},
    view::greeting_view,
};

#[derive(Debug)]
pub(crate) struct GreetingScreen {
    state: State,
}

impl GreetingScreen {
    pub(crate) fn new(person_name: String) -> Self {
        GreetingScreen {
            state: State {
                model: Model { person_name },
                view_state: ViewState {
                    button_state: button::State::default(),
                },
            },
        }
    }
}

impl Screen<RouteMessage, ScreenMessage> for GreetingScreen {
    fn title(&self) -> String {
        format!("Hello {}", self.state.model.person_name)
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
        greeting_view(&self.state.model, &mut self.state.view_state)
    }
}
