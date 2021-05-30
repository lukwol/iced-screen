use iced::{button, text_input};

#[derive(Debug, Default)]
pub(super) struct Model {
    pub(super) person_name: String,
}

#[derive(Debug, Default)]
pub(super) struct ViewState {
    pub(super) button_state: button::State,
    pub(super) text_input_state: text_input::State,
}

#[derive(Debug, Default)]
pub(super) struct State {
    pub(super) model: Model,
    pub(super) view_state: ViewState,
}
