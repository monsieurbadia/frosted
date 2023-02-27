pub use super::message::Message;
pub use super::page::{Page, Pages};

use std::sync::mpsc::Receiver;

pub struct Settings {
  pub title: String,
  pub pages: Pages,
  pub receiver: Receiver<Message>,
}

impl Settings {
  pub fn new<T: Into<String>>(
    title: T,
    pages: Pages,
    receiver: Receiver<Message>,
  ) -> Self {
    Self {
      title: title.into(),
      pages,
      receiver,
    }
  }
}
