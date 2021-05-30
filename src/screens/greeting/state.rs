use iced::button;

#[derive(Debug)]
pub(super) struct Model {
    pub(super) person_name: String,
}

#[derive(Debug)]
pub(super) struct ViewState {
    pub(super) button_state: button::State,
}

#[derive(Debug)]
pub(super) struct State {
    pub(super) model: Model,
    pub(super) view_state: ViewState,
}
