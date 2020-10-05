# libinput-gestures-macos

Emulates macOS-style swipes for forward and backward.

On Linux, libinput will normally interpret this as horizontal scrolling, which isn't wrong, but if you're like me, the muscle memory to switch from two fingers to three fingers can be hard.  This program will track horizontal scrolling activity and calculate the velocity of the scrolling event, triggering actions when it quantifies the scrolling as a full-fledged swipe.

## warning

This program is very rough, totally hard-coded, and is designed purely (right now) to emulate macOS two-finger swipes, nothing more.

~~- Hard-coded to send `alt+Right` or `alt+Left` for left and right swipes, respectively.~~
~~- Hard-coded to a specific nput device. (My touchpad on my particular laptop.)~~
~~- Hard-coded velocity threshold. (This is likely fine for most people, though.)~~

~~Requires [`xdotool`](https://github.com/jordansissel/xdotool) to actually send the commands to the OS.~~

Please note that, my version of this `libinput-gestures-macos` is based on [Tobz](https://github.com/tobz/libinput-gestures-macos)'s implementation, with some bug fix, configuration settings, refactoring.

This is my first time coding in `Rust`, the code may not clean and optimal enough.

## tech specs

Based on the [`input`](https://github.com/Smithay/input.rs) crate to parse `libinput` events, and [`mio`](https://github.com/tokio-rs/mio) and [`tokio`](https://github.com/tokio-rs/tokio) to asynchronously listen to the libinput data.

## usage

1.) Figure out which input device is your touchpad. This will be a `/dev/input/eventXX` device.

For the reason of the event id will change on every reboot, we need to use the symlink generated by the system instead.

First of all, we can find the event id by using `evtest` (lookup the device you want and the `/dev/input/eventXX` event id is listed at the left). Then use `ls -lah /dev/input/by-path` to find out which symlink is point to the right event id.

2.) Checkout the project.

```bash
git clone https://github.com/alsan/libinput-gestures-macos
cd libinput-gestures-macos
```

3.) Create a config file under `$XDG_CONFIG_HOME/libinput-gestures-macos` (that would be in `$HOME/.config/` in most recent linux system) named config.ini, the content would be similar to the example config in `example` directory of the source.

4.) Build and run the project.  (You need [Rust](https://www.rust-lang.org/tools/install) for this.)

```bash
cargo build --release
target/release/libinput-gestures-macos
```

6.) Switch to another window -- a browser is ideal obviously -- and you should be able to two-finger swipe to go forwards and backwards.

7.) Making this run on system boot, etc, is an exercise left to the reader.

## open source

PRs welcome for any improvements.  MIT license, so do whatever you want.
