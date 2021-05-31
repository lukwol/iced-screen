use std::fmt::Debug;

use iced::{Command, Element};

use super::message::Message;

pub trait Screen<RouteMessage, ScreenMessage, GlobalMessage = ()>: Debug + Send {
    fn title(&self) -> String {
        "".to_string()
    }

    fn update(
        &mut self,
        message: ScreenMessage,
        clipboard: &mut iced::Clipboard,
    ) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>>;

    fn global_update(&mut self, _message: GlobalMessage, _clipboard: &mut iced::Clipboard) {}

    fn on_push(&mut self) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }
    fn on_push_stack(&mut self) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }
    fn on_pop(&mut self) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }
    fn on_pop_stack(&mut self) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message<RouteMessage, ScreenMessage, GlobalMessage>>;
}

pub trait InitialScreen<RouteMessage, ScreenMessage, GlobalMessage = (), Flags = ()>:
    Screen<RouteMessage, ScreenMessage, GlobalMessage>
{
    #[allow(clippy::new_ret_no_self)]
    fn new(
        flags: Flags,
    ) -> (
        Box<dyn Screen<RouteMessage, ScreenMessage, GlobalMessage>>,
        Command<Message<RouteMessage, ScreenMessage, GlobalMessage>>,
    )
    where
        Self: Sized;
}
