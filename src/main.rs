#![allow(clippy::type_complexity)]
use common::{messages::screen::ScreenMessage, router::Router};
use iced::{Application, Settings};
use routing::app::RoutedApp;

mod common;
mod screens;

fn main() {
    let _ = RoutedApp::<Router, ScreenMessage>::run(Settings::default());
}
