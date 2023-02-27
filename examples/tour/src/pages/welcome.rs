use frosted::pager::message::Message;
use frosted::pager::page::Page;

use iced::widget::{Button, Column, Text};
use iced::Element;

use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct WelcomePage {
  pub sender: Sender<Message>,
}

impl WelcomePage {
  pub fn new(sender: Sender<Message>) -> Self {
    Self { sender }
  }
}

impl Page for WelcomePage {
  fn name(&self) -> String {
    "welcome".into()
  }

  fn sender(&self) -> &Sender<Message> {
    &self.sender
  }

  fn view(&self) -> Element<'_, Message> {
    Column::new()
      .push(Text::new("this is the page one").size(50))
      .spacing(20)
      .push(
        "This is a simple tour meant to showcase a bunch of widgets \
                 that can be easily implemented on top of Iced.",
      )
      .push(
        "Iced is a cross-platform GUI library for Rust focused on \
                 simplicity and type-safety. It is heavily inspired by Elm.",
      )
      .push(
        "It was originally born as part of Coffee, an opinionated \
                 2D game engine for Rust.",
      )
      .push(
        "On native platforms, Iced provides by default a renderer \
                 built on top of wgpu, a graphics library supporting Vulkan, \
                 Metal, DX11, and DX12.",
      )
      .push(
        "Additionally, this tour can also run on WebAssembly thanks \
                 to dodrio, an experimental VDOM library for Rust.",
      )
      .push(
        "You will need to interact with the UI in order to reach the \
                 end!",
      )
      .push(
        Button::new(Text::new("next"))
          .on_press(Message::ChangePage("slider".into())),
      )
      .into()
  }
}
