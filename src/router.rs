use std::fmt::Debug;

use iced::Command;

use super::{message::Message, screen::Screen};

pub trait Router<ScreenMessage, Flags = ()> {
    type RouteMessage: Debug + Clone + Send;

    fn initial_screen(
        flags: Flags,
    ) -> (
        Box<dyn Screen<Self::RouteMessage, ScreenMessage>>,
        Command<Message<Self::RouteMessage, ScreenMessage>>,
    );

    fn screen(message: Self::RouteMessage) -> Box<dyn Screen<Self::RouteMessage, ScreenMessage>>;
}
