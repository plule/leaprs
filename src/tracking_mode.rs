use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum TrackingMode {
    #[doc = " The tracking mode optimised for desktop devices @since 5.0.0"]
    Desktop = _eLeapTrackingMode_eLeapTrackingMode_Desktop,
    #[doc = " The tracking mode optimised for head-mounted devices @since 5.0.0"]
    HMD = _eLeapTrackingMode_eLeapTrackingMode_HMD,
    #[doc = " The tracking mode optimised for screen top-mounted devices @since 5.0.0"]
    ScreenTop = _eLeapTrackingMode_eLeapTrackingMode_ScreenTop,
    #[doc = " Tracking mode is not known (allows triggering of a new LEAP_TRACKING_MODE_EVENT) @since 5.0.0"]
    Unknown = _eLeapTrackingMode_eLeapTrackingMode_Unknown,
    #[doc = " The tracking mode is not recognized. This is likely a bug or a software version mismatch."]
    #[num_enum(default)]
    Unsupported,
}
