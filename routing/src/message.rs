use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Message<RouteMessage, ScreenMessage, GlobalMessage = ()> {
    Global(GlobalMessage),
    Local(ScreenMessage),
    Navigate {
        route: RouteMessage,
        navigation_type: NavigationType,
    },
    PopScreenStack,
    PopScreen,
}

#[derive(Debug, Clone)]
pub enum NavigationType {
    PushScreen,
    PushScreenStack,
}
