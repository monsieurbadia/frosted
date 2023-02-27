use frosted::mpsc;
use frosted::mpsc::sender::Sender;
use frosted::pager::message::Message;
use frosted::pager::page::Page;
use frosted::pager::Pager;

use iced::widget::{Button, Column, Text};
use iced::{Element, Result, Sandbox, Settings};

#[derive(Debug)]
struct HelloPage {
  sender: Sender<Message>,
}

impl Page for HelloPage {
  fn name(&self) -> String {
    "hello".into()
  }

  fn sender(&self) -> &Sender<Message> {
    &self.sender
  }

  fn view(&self) -> Element<'_, Message> {
    Column::new()
      .push(Text::new(self.name()).size(150))
      .push(
        Button::new(Text::new("next")).on_press(Message::ChangePage("world")),
      )
      .into()
  }
}

#[derive(Debug)]
struct WorldPage {
  sender: Sender<Message>,
}

impl Page for WorldPage {
  fn name(&self) -> String {
    "world".into()
  }

  fn sender(&self) -> &Sender<Message> {
    &self.sender
  }

  fn view(&self) -> Element<'_, Message> {
    Column::new()
      .push(Text::new(self.name()).size(150))
      .push(
        Button::new(Text::new("next")).on_press(Message::ChangePage("hello")),
      )
      .into()
  }
}

struct Hello {
  pager: Pager,
}

impl Sandbox for Hello {
  type Message = Message;

  fn new() -> Self {
    let (sender, receiver) = mpsc::channel();

    let pager = Pager::new(receiver)
      .page_current("hello")
      .add_page(Box::new(HelloPage {
        sender: sender.clone(),
      }))
      .add_page(Box::new(WorldPage {
        sender: sender.clone(),
      }));

    Self { pager }
  }

  fn title(&self) -> String {
    "hello world".into()
  }

  fn update(&mut self, message: Self::Message) {
    self.pager.update(message);
  }

  fn view(&self) -> Element<Self::Message> {
    self.pager.view()
  }
}

fn main() -> Result {
  Hello::run(Settings::default())
}
