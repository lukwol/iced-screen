use std::fmt::Debug;

use iced::Command;

use super::{message::Message, screen::Screen};

pub trait Router<ScreenMessage, GlobalMessage = (), Flags = ()> {
    type RouteMessage: Debug + Clone + Send;

    fn initial_screen(
        flags: Flags,
    ) -> (
        Box<dyn Screen<Self::RouteMessage, ScreenMessage, GlobalMessage>>,
        Command<Message<Self::RouteMessage, ScreenMessage, GlobalMessage>>,
    );

    fn screen(
        message: Self::RouteMessage,
    ) -> Box<dyn Screen<Self::RouteMessage, ScreenMessage, GlobalMessage>>;
}
