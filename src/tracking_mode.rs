use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
#[doc = " Enumerates values for the tracking mode."]
pub enum TrackingMode {
    #[doc = " The tracking mode optimised for desktop devices @since 5.0.0"]
    Desktop = _eLeapTrackingMode_eLeapTrackingMode_Desktop,
    #[doc = " The tracking mode optimised for head-mounted devices @since 5.0.0"]
    HMD = _eLeapTrackingMode_eLeapTrackingMode_HMD,
    #[doc = " The tracking mode optimised for screen top-mounted devices @since 5.0.0"]
    ScreenTop = _eLeapTrackingMode_eLeapTrackingMode_ScreenTop,
    #[doc = " Tracking mode is not known (allows triggering of a new LEAP_TRACKING_MODE_EVENT) @since 5.0.0"]
    #[num_enum(default)]
    Unknown = _eLeapTrackingMode_eLeapTrackingMode_Unknown,
}
