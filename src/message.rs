use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Message<RouteMessage, ScreenMessage, GlobalMessage = ()> {
    Local(ScreenMessage),
    Global(GlobalMessage),
    Navigate(NavigateMessage<RouteMessage>),
}

#[derive(Debug, Clone)]
pub enum NavigateMessage<RouteMessage> {
    Route {
        route: RouteMessage,
        navigation_type: NavigationType,
    },
    PopScreenStack,
    PopScreen,
}

#[derive(Debug, Clone)]
pub enum NavigationType {
    PushScreenStack,
    PushScreen,
}
