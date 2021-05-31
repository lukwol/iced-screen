use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use iced::{executor, Application, Command, Element};
use screen::Screen;

use super::{
    message::{Message, NavigationType},
    router, screen,
};

type ScreenStack<RouteMessage, ScreenMessage, GlobalMessage = ()> =
    Vec<Box<dyn Screen<RouteMessage, ScreenMessage, GlobalMessage>>>;

pub struct RoutedApp<
    Router,
    ScreenMessage,
    GlobalMessage = (),
    Flags = (),
    Executor = executor::Default,
> where
    Router: router::Router<ScreenMessage, GlobalMessage, Flags>,
{
    screen_stacks: Vec<ScreenStack<Router::RouteMessage, ScreenMessage, GlobalMessage>>,
    router: PhantomData<Router>,
    executor: PhantomData<Executor>,
}

impl<Router, ScreenMessage, GlobalMessage, Executor, Flags>
    RoutedApp<Router, ScreenMessage, GlobalMessage, Flags, Executor>
where
    Router: router::Router<ScreenMessage, GlobalMessage, Flags>,
{
    fn top_screen_stack(&self) -> &ScreenStack<Router::RouteMessage, ScreenMessage, GlobalMessage> {
        self.screen_stacks
            .last()
            .expect("Application doesn't have any screen stacks")
    }

    fn top_screen_stack_mut(
        &mut self,
    ) -> &mut ScreenStack<Router::RouteMessage, ScreenMessage, GlobalMessage> {
        self.screen_stacks
            .last_mut()
            .expect("Application doesn't have any screen stacks")
    }

    fn top_screen(&self) -> &dyn Screen<Router::RouteMessage, ScreenMessage, GlobalMessage> {
        self.top_screen_stack()
            .last()
            .expect("Application doesn't have any screens")
            .deref()
    }

    fn top_screen_mut(
        &mut self,
    ) -> &mut dyn Screen<Router::RouteMessage, ScreenMessage, GlobalMessage> {
        self.top_screen_stack_mut()
            .last_mut()
            .expect("Application doesn't have any screens")
            .deref_mut()
    }
}

impl<Router, ScreenMessage, GlobalMessage, Flags, Executor> Application
    for RoutedApp<Router, ScreenMessage, GlobalMessage, Flags, Executor>
where
    Router: router::Router<ScreenMessage, GlobalMessage, Flags>,
    ScreenMessage: Debug + Clone + Send,
    GlobalMessage: Debug + Clone + Copy + Send,
    Executor: iced::Executor,
{
    type Executor = Executor;

    type Message = Message<Router::RouteMessage, ScreenMessage, GlobalMessage>;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (mut screen, command) = Router::initial_screen(flags);
        let mut commands = Vec::new();
        commands.push(screen.on_create());
        commands.push(screen.on_present());
        commands.push(command);
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
            Message::Global(msg) => Command::batch(
                self.screen_stacks
                    .iter_mut()
                    .flatten()
                    .map(|screen| screen.global_update(msg, clipboard)),
            ),
            Message::Local(msg) => self.top_screen_mut().update(msg, clipboard),
            Message::Navigate {
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
            Message::PopScreenStack => {
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
            Message::PopScreen => {
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
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        self.top_screen_mut().view()
    }
}
