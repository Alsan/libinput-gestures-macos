use futures::future::poll_fn;
use input::{event::Event, Libinput};
use mio::Ready;
use std::os::unix::io::AsRawFd;
use std::task::Poll as FuturesPoll;
use tokio::io::PollEvented;

mod eventd;
mod interface;

// Wrapper for libinput context that handles the asynchronous aspect.
pub struct LibinputContext(pub Libinput, pub PollEvented<eventd::LibinputEvented>);

impl LibinputContext {
  pub fn open<P>(p: P) -> Result<LibinputContext, ()>
  where
    P: AsRef<str>,
  {
    let mut context = Libinput::new_from_path(interface::BasicLibinputInterface);
    if let None = context.path_add_device(p.as_ref()) {
      return Err(());
    }

    let ev = PollEvented::new(eventd::LibinputEvented(context.as_raw_fd())).map_err(|_err| ())?;

    Ok(LibinputContext(context, ev))
  }

  pub fn resume(&mut self) -> Result<(), ()> {
    self.0.resume()
  }

  pub async fn next(&mut self) -> Result<Event, ()> {
    loop {
      let _ = self.0.dispatch().map_err(|_| ())?;

      match self.0.next() {
        Some(e) => return Ok(e),
        None => {
          let _ = poll_fn(|cx| self.1.poll_read_ready(cx, Ready::readable()))
            .await
            .map_err(|_| ())?;
          let _ = poll_fn(|cx| FuturesPoll::Ready(self.1.clear_read_ready(cx, Ready::readable())))
            .await
            .map_err(|_| ())?;
        }
      }
    }
  }
}
