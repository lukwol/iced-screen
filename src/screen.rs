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
        update_children(self, message, clipboard)
    }

    fn on_create(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        on_create_children(self)
    }

    fn on_present(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        on_present_children(self)
    }

    fn on_stop_presenting(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        on_stop_presenting_children(self)
    }

    fn on_dismiss(&mut self) -> Command<Message<RouteMessage, ScreenMessage>> {
        on_dismiss_children(self)
    }

    fn view(&mut self) -> Element<Message<RouteMessage, ScreenMessage>>;
}

pub fn update_children<T, RouteMessage, ScreenMessage>(
    screen: &mut T,
    message: ScreenMessage,
    clipboard: &mut iced::Clipboard,
) -> Command<Message<RouteMessage, ScreenMessage>>
where
    T: Screen<RouteMessage, ScreenMessage> + ?Sized,
    ScreenMessage: Clone,
{
    println!("Calling update children!");
    Command::batch(
        screen
            .child_screens()
            .iter_mut()
            .map(|screen| screen.update(message.clone(), clipboard)),
    )
}

pub fn on_create_children<T, RouteMessage, ScreenMessage>(
    screen: &mut T,
) -> Command<Message<RouteMessage, ScreenMessage>>
where
    T: Screen<RouteMessage, ScreenMessage> + ?Sized,
    ScreenMessage: Clone,
{
    Command::batch(
        screen
            .child_screens()
            .iter_mut()
            .map(|screen| screen.on_create()),
    )
}

pub fn on_present_children<T, RouteMessage, ScreenMessage>(
    screen: &mut T,
) -> Command<Message<RouteMessage, ScreenMessage>>
where
    T: Screen<RouteMessage, ScreenMessage> + ?Sized,
    ScreenMessage: Clone,
{
    Command::batch(
        screen
            .child_screens()
            .iter_mut()
            .map(|screen| screen.on_present()),
    )
}

pub fn on_stop_presenting_children<T, RouteMessage, ScreenMessage>(
    screen: &mut T,
) -> Command<Message<RouteMessage, ScreenMessage>>
where
    T: Screen<RouteMessage, ScreenMessage> + ?Sized,
    ScreenMessage: Clone,
{
    Command::batch(
        screen
            .child_screens()
            .iter_mut()
            .map(|screen| screen.on_stop_presenting()),
    )
}

pub fn on_dismiss_children<T, RouteMessage, ScreenMessage>(
    screen: &mut T,
) -> Command<Message<RouteMessage, ScreenMessage>>
where
    T: Screen<RouteMessage, ScreenMessage> + ?Sized,
    ScreenMessage: Clone,
{
    Command::batch(
        screen
            .child_screens()
            .iter_mut()
            .map(|screen| screen.on_dismiss()),
    )
}
