use iced::{Align, Button, Column, Container, Element, Length, Text, TextInput};
use routing::message::{Message, NavigationType};

use crate::common::messages::{route::RouteMessage, screen::ScreenMessage};

use super::{
    state::{Model, ViewState},
    InputScreenMessage,
};

pub(super) fn input_view<'a>(
    model: &'a Model,
    view_state: &'a mut ViewState,
) -> Element<'a, Message<RouteMessage, ScreenMessage>> {
    Container::new(
        Column::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(
                Text::new("Greet Me!")
                    .size(30),
            )
            .push(
                TextInput::new(
                    &mut view_state.text_input_state,
                    "Enter your name",
                    &model.person_name,
                    |text| {
                        Message::Local(ScreenMessage::InputScreen(
                            InputScreenMessage::PersonNameChanged(text),
                        ))
                    },
                )
                .width(Length::Units(200))
                .on_submit(Message::Navigate {
                    route: RouteMessage::GreetingScreenRoute {
                        person_name: model.person_name.to_string(),
                    },
                    navigation_type: NavigationType::PushScreen,
                }),
            )
            .push(
                Button::new(&mut view_state.button_state, Text::new("Go!")).on_press(
                    Message::Navigate {
                        route: RouteMessage::GreetingScreenRoute {
                            person_name: model.person_name.to_string(),
                        },
                        navigation_type: NavigationType::PushScreen,
                    },
                ),
            ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .into()
}
