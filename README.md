# LeapRS

[![Github](https://img.shields.io/badge/github-plule%2Fleaprs-8da0cb?style=flat-square)](https://github.com/plule/leaprs)
[![Crates.io](https://img.shields.io/crates/v/leaprs?style=flat-square)](https://crates.io/crates/leaprs)
[![docs.rs](https://img.shields.io/docsrs/leaprs?style=flat-square)](https://docs.rs/leaprs)
![Crates.io](https://img.shields.io/crates/l/leaprs?style=flat-square)

LeapRS is a safe wrapper for LeapC, the [Leap Motion C
API](https://docs.ultraleap.com/tracking-api/). It uses the generated binding
provided by [leap-sys](https://crates.io/crates/leap-sys).

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

`cargo add leaprs`

You also need to install the [LeapMotion Tracking
Software](https://developer.leapmotion.com/tracking-software-download).

This library was created for the version named `Geminy` (5.6.1).

If you install this software in a custom path, you need to define the
environment variable `LEAPSDK_LIB_PATH` (default: `C:\Program
Files\Ultraleap\LeapSDK\lib\x64`).

The Ultraleap backend is only compatible with Windows, and so is LeapRS. Older
versions of the software were multi-platform, so it could be possible to create
a more compatible library, but it is not in the scope at the moment.

## Runtime

At runtime, the application requires the LeapC.dll file to be available. The
easiest way to do it during development is to add its folder to the `PATH`
environment variable. For distribution, refer to the SDK licensing.

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

## Implementation

The enum safety is provided through [num_enum](https://docs.rs/num_enum/latest/num_enum/).

The bitflags are wrapped using [bitflags](https://docs.rs/bitflags/latest/bitflags/).

Most struct are simple wrappers around their C counter-part. Most of the time,
the C struct is the only member of the wrapper, except when external allocation
is needed (when providing a pre-allocated array to LeapC). Accessors are then
exposing all the functions and members associated with these structs in a
consistent way.

## Tests

The test require to have the Leap software up and running, and to have a device
connected.
