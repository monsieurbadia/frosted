use tour::pages::{SliderPage, WelcomePage};

use frosted::pager::message::Message;
use frosted::pager::Pager;

use iced::{Element, Sandbox};

use std::fmt::Debug;
#[derive(Debug, Clone)]
pub enum StepMessage {
  SliderChanged(u8),
}

struct Tour {
  pager: Pager,
}

impl Sandbox for Tour {
  type Message = Message;

  fn new() -> Self {
    let (sender, receiver) = std::sync::mpsc::channel();

    let pager = Pager::new(receiver)
      .page_current("welcome")
      .add_page(Box::new(WelcomePage {
        sender: sender.clone(),
      }))
      .add_page(Box::new(SliderPage {
        sender: sender.clone(),
        value: 0,
      }));

    Self { pager }
  }

  fn title(&self) -> String {
    String::from("frosted tour!")
  }

  fn update(&mut self, message: Self::Message) {
    self.pager.update(message)
  }

  fn view(&self) -> Element<Self::Message> {
    self.pager.view()
  }
}

fn main() -> iced::Result {
  Tour::run(iced::Settings::default())
}
