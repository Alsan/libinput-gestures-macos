use input::{
    event::pointer::{Axis, AxisSource, PointerEvent, PointerEventTrait},
    event::Event,
};
use std::io::Error;
use std::process::{Command, Stdio};

mod config;
mod context;
mod tracking;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = config::Config::default().construct();

    let left_action: Vec<&str> = settings.left_swipe_action.split_whitespace().collect();
    let left_cmd = left_action[0];
    let left_args = left_action.get(1..).unwrap();

    let right_action: Vec<&str> = settings.right_swipe_action.split_whitespace().collect();
    let right_cmd = right_action[0];
    let right_args = right_action.get(1..).unwrap();

    let mut rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        let mut context =
            context::LibinputContext::open(settings.device.as_str()).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "failed to create libinput context",
                )
            })?;

        context.resume().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "failed to resume libinput context",
            )
        })?;

        let mut left_swipe = tracking::SwipeTracking::new();
        let mut right_swipe = tracking::SwipeTracking::new();

        while let Ok(e) = context.next().await {
            match e {
                // We only care about horizontal scroll pointer events, which are generated when
                // libinput detects two-finger swipes.
                Event::Pointer(PointerEvent::Axis(pae)) => {
                    if pae.has_axis(Axis::Horizontal) && pae.axis_source() == AxisSource::Finger {
                        // Track which direction the swipe is going.
                        let av = pae.axis_value(Axis::Horizontal);

                        if av < 0.0 {
                            left_swipe.measure_event(pae.time_usec(), av);
                        } else if av > 0.0 {
                            right_swipe.measure_event(pae.time_usec(), av);
                        } else {
                            // No magnitude for the swipe action, which is a special signal that the
                            // swipe has stopped.  Calculate based on our running total if we should
                            // actually treat this as a swipe, based on our velocity threshold.
                            let tend = pae.time_usec();
                            let lvdelta = left_swipe.flush(tend);
                            let rvdelta = right_swipe.flush(tend);

                            // We reverse the direction to emulate natural scrolling: if you drag your
                            // fingers right to left (left swipe), you're swiping the page to the left,
                            // or pulling the next page to you, and vise versa.  Just like a book.
                            let result = if lvdelta.is_some() && rvdelta.is_none() {
                                Some((lvdelta.unwrap(), left_cmd, left_args))
                            } else if rvdelta.is_some() && lvdelta.is_none() {
                                Some((rvdelta.unwrap(), right_cmd, right_args))
                            } else {
                                None
                            };

                            // This cancels out weird events where the user scrolled/swiped both left
                            // and right or their touchpad picked up something weird.
                            if let Some((vdelta, cmd, cmd_args)) = result {
                                if vdelta.abs() >= settings.threshold {
                                    let _ = launch_xdotool(cmd, cmd_args);
                                }
                            }
                        }
                    }
                }
                // We only handle pointer events.
                _ => {}
            }
        }

        Ok(())
    })
}

fn launch_xdotool(cmd: &str, cmd_opts: &[&str]) -> Result<(), Error> {
    Command::new(cmd)
        .args(cmd_opts)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
