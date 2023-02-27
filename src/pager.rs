pub mod message;
pub mod page;
pub mod settings;

use super::mpsc::receiver::Receiver;

use message::Message;
use page::{Page, Pages};

use iced::Element;

#[derive(Debug)]
pub struct Pager {
  page_current: String,
  pub pages: Pages,
  receiver: Receiver<Message>,
}

impl Pager {
  pub fn new(receiver: Receiver<Message>) -> Self {
    Self {
      page_current: String::new(),
      pages: Pages::new(),
      receiver,
    }
  }

  pub fn page_current(mut self, page_current: impl Into<String>) -> Self {
    self.page_current = page_current.into();
    self
  }

  pub fn add_page(mut self, page: Box<dyn Page>) -> Self {
    self.pages.add_page(page);

    self
  }

  pub fn update(&mut self, message: Message) {
    if self.pages.is_empty() {
      return;
    }

    match message {
      Message::ChangePage(page_current) => {
        if let Some(page) = self.pages.get_page(page_current) {
          page.update(Message::ChangePage(page_current));
        }
      }
    }

    while let Ok(message) = self.receiver.try_recv() {
      match message {
        Message::ChangePage(page_current) => {
          self.page_current = page_current.into();
        }
      }
    }
  }

  pub fn view(&self) -> Element<'_, Message> {
    if self.pages.is_empty() {
      panic!("no pages has been initialized");
    }

    match self.pages.get_page(&self.page_current) {
      Some(page) => page.view(),
      None => panic!("page name `{}`do not exist", self.page_current),
    }
  }
}
