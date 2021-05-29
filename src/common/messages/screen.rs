use crate::screens::greeting::GreetingScreenMessage;
use crate::screens::input::InputScreenMessage;

#[derive(Debug, Clone)]
pub(crate) enum ScreenMessage {
    InputScreen(InputScreenMessage),
    #[allow(dead_code)]
    GreetingScreen(GreetingScreenMessage),
}
