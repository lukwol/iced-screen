use std::fmt::Debug;

use iced::{Command, Element};

use super::message::Message;

pub trait Screen<RouteMessage, ScreenMessage, GlobalMessage = ()>: Debug + Send {
    fn title(&self) -> String {
        "".to_string()
    }

    fn update(
        &mut self,
        _message: ScreenMessage,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }

    fn global_update(
        &mut self,
        _message: GlobalMessage,
        _clipboard: &mut iced::Clipboard,
    ) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }

    fn on_create(&mut self) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }

    fn on_present(&mut self) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }

    fn on_stop_presenting(
        &mut self,
    ) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
        Command::none()
    }

    fn on_dismiss(&mut self) -> Command<Message<RouteMessage, ScreenMessage, GlobalMessage>> {
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
