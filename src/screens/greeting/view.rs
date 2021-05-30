use iced::{Align, Button, Column, Container, Element, Length, Text};
use routing::message::Message;

use crate::common::messages::{route::RouteMessage, screen::ScreenMessage};

use super::state::{Model, ViewState};

pub(super) fn greeting_view<'a>(
    model: &'a Model,
    view_state: &'a mut ViewState,
) -> Element<'a, Message<RouteMessage, ScreenMessage>> {
    Container::new(
        Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new(format!("Hello {}", model.person_name)).height(Length::Units(20)))
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
