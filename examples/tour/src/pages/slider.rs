use frosted::pager::message::Message;
use frosted::pager::page::Page;
use iced::alignment::Horizontal;

use iced::widget::{Button, Column, Text};
use iced::{Element, Length};

use std::sync::mpsc::Sender;

#[derive(Debug)]
pub struct SliderPage {
  pub sender: Sender<Message>,
  // add your own props
  pub value: u8,
}

impl SliderPage {
  pub fn new(sender: Sender<Message>) -> Self {
    Self { sender, value: 0 }
  }
}

impl Page for SliderPage {
  fn name(&self) -> String {
    "slider".into()
  }

  fn sender(&self) -> &Sender<Message> {
    &self.sender
  }

  fn view(&self) -> Element<'_, Message> {
    Column::new()
      .push(Text::new("this is the page two"))
      .push(
        "A slider allows you to smoothly select a value from a range \
         of values.",
      )
      .push(
        "The following slider lets you choose an integer from \
         0 to 100:",
      )
      .push(
        Text::new(self.value.to_string())
          .width(Length::Fill)
          .horizontal_alignment(Horizontal::Center),
      )
      .push(
        Button::new(Text::new("move"))
          .on_press(Message::ChangePage("welcome".into())),
      )
      .into()
  }
}
