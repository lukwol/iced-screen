use iced::{Align, Button, Column, Container, Element, Length, Text};
use routing::message::Message;

use crate::common::messages::{route::RouteMessage, screen::ScreenMessage};

use super::state::{Model, ViewState};
use rand::seq::SliceRandom;

const GREETINGS: &[&str] = &["Bonjour", "Hola", "Olá", "Ciao", "Hi", "Hallo", "Hey"];

fn greeting() -> &'static str {
    GREETINGS.choose(&mut rand::thread_rng()).unwrap()
}

pub(super) fn greeting_view<'a>(
    model: &'a Model,
    view_state: &'a mut ViewState,
) -> Element<'a, Message<RouteMessage, ScreenMessage>> {
    Container::new(
        Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new(format!("{} {}", greeting(), model.person_name)).size(40))
            .push(
                Button::new(&mut view_state.button_state, Text::new("Go Back!"))
                    .on_press(Message::PopScreen),
            ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}
