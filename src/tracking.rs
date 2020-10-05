// Tracks the velocity of a swipe.
pub struct SwipeTracking {
  pub tstart: u64,
  pub vtotal: f64,
}

impl SwipeTracking {
  pub fn new() -> SwipeTracking {
    SwipeTracking {
      tstart: 0,
      vtotal: 0.0,
    }
  }

  pub fn measure_event(&mut self, t: u64, v: f64) {
    if self.tstart == 0 {
      self.tstart = t;
    }
    self.vtotal += v;
  }

  pub fn flush(&mut self, t: u64) -> Option<f64> {
    if self.tstart == 0 {
      return None;
    }

    let tdelta = t - self.tstart;
    let vdelta = self.vtotal / tdelta as f64;
    self.tstart = 0;
    self.vtotal = 0.0;
    Some(vdelta)
  }
}
