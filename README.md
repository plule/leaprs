# LeapRS

LeapRS is a safe wrapper for LeapC, the [Leap Motion C
API](https://docs.ultraleap.com/tracking-api/).

This is an API for accessing Leap Motion/Ultraleap hand tracking device. It only
works on Windows.

<div class="example-wrap" style="display:inline-block"><pre class="compile_fail" style="white-space:normal;font:inherit;">

**Warning**: This library is not complete and not fully tested. Moreover, it includes unsafe
code to interact with the C API. It should be treated with caution in its
current state. It has not yet been used in any actual application.

</pre></div>

## Scope

The goal of LeapRS is to cover entirely LeapC in a safe way. It is intended to
be as close as possible to the original C library. As such, it's fairly low
level and carries most of the difficulties of using LeapC.

It is not intended to provide any full featured framework such as having a
worker thread and provide an async API. It is intended to be usable to create
such framework though.

### API Coverage

The current coverage is partial. It includes everything needed to capture hand
position in a single device setup. It misses multi-device management and most of
the image manipulation methods. It also misses interpolation methods.

## Installation

To build and run programs, you first need to install the [LeapMotion Tracking Software](https://developer.leapmotion.com/tracking-software-download).

This library was created for the version named `Geminy` (5.6.1).

If you install this software in a custom path, you need to define the
environment variable `LEAPSDK_LIB_PATH` (default: `C:\Program
Files\Ultraleap\LeapSDK\lib\x64`)

**Cargo.toml** TODO

## Quick start

```rust
use leaprs::*;

let mut connection =
    Connection::create(ConnectionConfig::default()).expect("Failed to connect");
connection.open().expect("Failed to open the connection");
connection.poll(1000).expect("First poll failed");

for _ in 0..10 {
    match connection
        .poll(1000)
        .expect("Failed to poll for events.")
        .event()
    {
        Event::Tracking(e) => println!("There are {} hand(s) in view", e.hands().len()),
        _ => {}
    }
}
```
