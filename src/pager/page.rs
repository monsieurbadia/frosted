use super::Message;

use iced::Element;

pub trait Page: std::fmt::Debug + Send + 'static {
  fn name(&self) -> String;
  fn sender(&self) -> &std::sync::mpsc::Sender<Message>;

  fn update(&self, message: Message) {
    let Message::ChangePage(page_id) = message;

    self
      .sender()
      .send(Message::ChangePage(page_id))
      .expect("page message should be sent");
  }

  fn view(&self) -> Element<'_, Message>;
}

#[derive(Debug, Default)]
pub struct Pages(pub std::collections::HashMap<String, Box<dyn Page>>);

impl Pages {
  pub fn new() -> Self {
    Self(std::collections::HashMap::with_capacity(8))
  }

  pub fn add_page(&mut self, page: impl Into<Box<dyn Page>>) -> &Self {
    let page = page.into();
    let name = page.name();

    self.0.insert(name, page);
    self
  }

  pub fn get_page(&self, name: &str) -> Option<&Box<dyn Page>> {
    match self.0.get(name) {
      Some(page) => Some(page),
      None => None,
    }
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl std::ops::Deref for Pages {
  type Target = std::collections::HashMap<String, Box<dyn Page>>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl std::fmt::Display for Pages {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self
      .iter()
      .fold(Ok(()), |ok, phase| ok.and_then(|_| write!(f, "{phase:?}")))
  }
}
