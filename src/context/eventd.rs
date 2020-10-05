use mio::unix::EventedFd;
use mio::{event::Evented, Poll, PollOpt, Ready, Token};
use std::os::unix::io::RawFd;

// Eventd wrapper for the underlying libinput FD.
pub struct LibinputEvented(pub RawFd);

impl Evented for LibinputEvented {
  fn register(
    &self,
    poll: &Poll,
    token: Token,
    interest: Ready,
    opts: PollOpt,
  ) -> std::io::Result<()> {
    EventedFd(&self.0).register(poll, token, interest, opts)
  }

  fn reregister(
    &self,
    poll: &Poll,
    token: Token,
    interest: Ready,
    opts: PollOpt,
  ) -> std::io::Result<()> {
    EventedFd(&self.0).reregister(poll, token, interest, opts)
  }

  fn deregister(&self, poll: &Poll) -> std::io::Result<()> {
    EventedFd(&self.0).deregister(poll)
  }
}
