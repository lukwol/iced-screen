use std::fmt::Debug;

use iced::{Command, Element};

use super::message::Message;

pub trait Screen<RouteMessage, ScreenMessage>: Debug + Send
where
    ScreenMessage: Clone,
{
    fn title(&self) -> String {
        "".to_string()
    }

    fn child_screens(&mut self) -> Vec<&mut dyn Screen<RouteMessage, ScreenMessage>> {
        vec![]
    }

    fn update(
        &mut self,
        message: ScreenMessage,
        clipboard: &mut iced::Clipboard,
    ) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::batch(
            self.child_screens()
                .iter_mut()
                .map(|screen| screen.update(message.clone(), clipboard)),
        )
    }

    fn on_create(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::batch(
            self.child_screens()
                .iter_mut()
                .map(|screen| screen.on_create()),
        )
    }

    fn on_present(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::batch(
            self.child_screens()
                .iter_mut()
                .map(|screen| screen.on_present()),
        )
    }

    fn on_stop_presenting(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::batch(
            self.child_screens()
                .iter_mut()
                .map(|screen| screen.on_stop_presenting()),
        )
    }

    fn on_dismiss(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        Command::batch(
            self.child_screens()
                .iter_mut()
                .map(|screen| screen.on_dismiss()),
        )
    }

    fn view(&mut self) -> Element<Message<RouteMessage, ScreenMessage>>;
}
