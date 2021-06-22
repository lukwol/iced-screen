use iced::{Align, Button, Column, Container, Element, Length, Space, Text, TextInput};
use iced_screen::message::{Message, NavigateMessage, NavigationType};

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
            .align_items(Align::Center)
            .push(Text::new("Greet Me!").size(30))
            .push(Space::new(Length::Units(0), Length::Units(30)))
            .push(
                TextInput::new(
                    &mut view_state.text_input_state,
                    "Enter your name",
                    &model.person_name,
                    |text| {
                        Message::Screen(ScreenMessage::InputScreen(
                            InputScreenMessage::PersonNameChanged(text),
                        ))
                    },
                )
                .padding(10)
                .width(Length::Units(200))
                .on_submit(Message::Navigate(NavigateMessage::Route {
                    route: RouteMessage::GreetingScreenRoute {
                        person_name: model.person_name.to_string(),
                    },
                    navigation_type: NavigationType::PushScreen,
                }))
                .style(view_state.theme),
            )
            .push(Space::new(Length::Units(0), Length::Units(30)))
            .push(
                Button::new(&mut view_state.button_state, Text::new("Go!"))
                    .on_press(Message::Navigate(NavigateMessage::Route {
                        route: RouteMessage::GreetingScreenRoute {
                            person_name: model.person_name.to_string(),
                        },
                        navigation_type: NavigationType::PushScreen,
                    }))
                    .padding(10)
                    .style(view_state.theme),
            ),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .style(view_state.theme)
    .into()
}
