use input::LibinputInterface;
use nix::{
  fcntl::{open, OFlag},
  sys::stat::Mode,
  unistd::close,
};
use std::os::unix::io::RawFd;
use std::path::Path;

// Basic libinput interface for opening/closing FDs.
pub struct BasicLibinputInterface;

impl LibinputInterface for BasicLibinputInterface {
  fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
    open(path, OFlag::from_bits_truncate(flags), Mode::empty())
      // TODO: we should derive errno from err here but there's no conversion
      // from Errnp to i32 for w/e god damn reason so...
      .map_err(|_err| 1)
  }

  fn close_restricted(&mut self, fd: RawFd) {
    let _ = close(fd);
  }
}
