use std::fmt::Debug;

use iced::{Command, Element};

use super::message::Message;

pub trait Screen<RouteMessage, ScreenMessage>: Debug + Send {
    fn title(&self) -> String {
        "".to_string()
    }

    fn update(
        &mut self,
        _message: ScreenMessage,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::none()
    }

    fn on_create(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::none()
    }

    fn on_present(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::none()
    }

    fn on_stop_presenting(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::none()
    }

    fn on_dismiss(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message<RouteMessage, ScreenMessage>>;
}

pub trait InitialScreen<RouteMessage, ScreenMessage, Flags = ()>:
    Screen<RouteMessage, ScreenMessage>
{
    #[allow(clippy::new_ret_no_self)]
    fn new(
        flags: Flags,
    ) -> (
        Box<dyn Screen<RouteMessage, ScreenMessage>>,
        Command<Message<RouteMessage, ScreenMessage>>,
    )
    where
        Self: Sized;
}
