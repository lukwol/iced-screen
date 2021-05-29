use std::fmt::Debug;

#[derive(Debug, Clone)]
pub(crate) enum RouteMessage {
    GreetingScreenRoute { person_name: String },
}
