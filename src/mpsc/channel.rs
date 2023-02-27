use super::receiver::Receiver;
use super::sender::Sender;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
  std::sync::mpsc::channel()
}
