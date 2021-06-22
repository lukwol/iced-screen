use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use iced::{executor, Application, Command, Element};
use screen::Screen;

use crate::message::NavigateMessage;

use super::{
    message::{Message, NavigationType},
    router, screen,
};

type ScreenStack<RouteMessage, ScreenMessage> = Vec<Box<dyn Screen<RouteMessage, ScreenMessage>>>;

pub struct RoutedApp<Router, ScreenMessage, Flags = (), Executor = executor::Default>
where
    Router: router::Router<ScreenMessage, Flags>,
{
    screen_stacks: Vec<ScreenStack<Router::RouteMessage, ScreenMessage>>,
    router: PhantomData<Router>,
    executor: PhantomData<Executor>,
}

impl<Router, ScreenMessage, Executor, Flags> RoutedApp<Router, ScreenMessage, Flags, Executor>
where
    Router: router::Router<ScreenMessage, Flags>,
{
    fn top_screen_stack(&self) -> &ScreenStack<Router::RouteMessage, ScreenMessage> {
        self.screen_stacks
            .last()
            .expect("Application doesn't have any screen stacks")
    }

    fn top_screen_stack_mut(&mut self) -> &mut ScreenStack<Router::RouteMessage, ScreenMessage> {
        self.screen_stacks
            .last_mut()
            .expect("Application doesn't have any screen stacks")
    }

    fn top_screen(&self) -> &dyn Screen<Router::RouteMessage, ScreenMessage> {
        self.top_screen_stack()
            .last()
            .expect("Application doesn't have any screens")
            .deref()
    }

    fn top_screen_mut(&mut self) -> &mut dyn Screen<Router::RouteMessage, ScreenMessage> {
        self.top_screen_stack_mut()
            .last_mut()
            .expect("Application doesn't have any screens")
            .deref_mut()
    }
}

impl<Router, ScreenMessage, Flags, Executor> Application
    for RoutedApp<Router, ScreenMessage, Flags, Executor>
where
    Router: router::Router<ScreenMessage, Flags>,
    ScreenMessage: Debug + Clone + Send,
    Executor: iced::Executor,
{
    type Executor = Executor;

    type Message = Message<Router::RouteMessage, ScreenMessage>;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (mut screen, command) = Router::initial_screen(flags);
        let commands = vec![screen.on_create(), screen.on_present(), command];
        (
            RoutedApp {
                screen_stacks: vec![vec![screen]],
                router: PhantomData,
                executor: PhantomData,
            },
            Command::batch(commands),
        )
    }

    fn title(&self) -> String {
        self.top_screen().title()
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut iced::Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Screen(msg) => Command::batch(
                self.screen_stacks
                    .iter_mut()
                    .flatten()
                    .map(|screen| screen.update(msg.clone(), clipboard)),
            ),
            Message::Navigate(msg) => match msg {
                NavigateMessage::Route {
                    route,
                    navigation_type,
                } => {
                    let mut commands = Vec::new();
                    match navigation_type {
                        NavigationType::PushScreen => {
                            commands.push(self.top_screen_mut().on_stop_presenting());
                            self.top_screen_stack_mut().push(Router::screen(route));
                            let top_screen_mut = self.top_screen_mut();
                            commands.push(top_screen_mut.on_create());
                            commands.push(top_screen_mut.on_present());
                        }
                        NavigationType::PushScreenStack => {
                            commands.push(self.top_screen_mut().on_stop_presenting());
                            self.screen_stacks.push(vec![Router::screen(route)]);
                            let top_screen_mut = self.top_screen_mut();
                            commands.push(top_screen_mut.on_create());
                            commands.push(top_screen_mut.on_present());
                        }
                    }
                    Command::batch(commands)
                }
                NavigateMessage::PopScreen => {
                    let mut commands = Vec::new();
                    if self.top_screen_stack_mut().len() > 1 {
                        let top_screen_mut = self.top_screen_mut();
                        commands.push(top_screen_mut.on_stop_presenting());
                        commands.push(top_screen_mut.on_dismiss());
                        self.top_screen_stack_mut().pop();
                        commands.push(self.top_screen_mut().on_present());
                    }
                    Command::batch(commands)
                }
                NavigateMessage::PopScreenStack => {
                    let mut commands = Vec::new();
                    if self.screen_stacks.len() > 1 {
                        let top_screen_mut = self.top_screen_mut();
                        commands.push(top_screen_mut.on_stop_presenting());
                        commands.push(top_screen_mut.on_dismiss());
                        self.screen_stacks.pop();
                        commands.push(self.top_screen_mut().on_present());
                    }
                    Command::batch(commands)
                }
            },
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.top_screen_mut().view()
    }
}
