use iced::button;

use crate::common::style::Theme;

#[derive(Debug)]
pub(super) struct Model {
    pub(super) person_name: String,
}

#[derive(Debug, Default)]
pub(super) struct ViewState {
    pub(super) button_state: button::State,
    pub(super) theme: Theme,
}

#[derive(Debug)]
pub(super) struct State {
    pub(super) model: Model,
    pub(super) view_state: ViewState,
}
