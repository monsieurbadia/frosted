#[derive(Debug, Clone)]
pub enum Message {
  ChangePage(&'static str),
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}
