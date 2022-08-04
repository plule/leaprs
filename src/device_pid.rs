use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
#[doc = " Device hardware types. @since 3.0.0"]
pub enum DevicePID {
    #[doc = " An unknown device that is compatible with the tracking software. @since 3.1.3"]
    #[num_enum(default)]
    Unknown = _eLeapDevicePID_eLeapDevicePID_Unknown,
    #[doc = " The Leap Motion Controller (the first consumer peripheral). @since 3.0.0"]
    Peripheral = _eLeapDevicePID_eLeapDevicePID_Peripheral,
    #[doc = " Internal research product codename \"Dragonfly\". @since 3.0.0"]
    Dragonfly = _eLeapDevicePID_eLeapDevicePID_Dragonfly,
    #[doc = " Internal research product codename \"Nightcrawler\". @since 3.0.0"]
    Nightcrawler = _eLeapDevicePID_eLeapDevicePID_Nightcrawler,
    #[doc = " Research product codename \"Rigel\". @since 4.0.0"]
    Rigel = _eLeapDevicePID_eLeapDevicePID_Rigel,
    #[doc = " The Ultraleap Stereo IR 170 (SIR170) hand tracking module. @since 5.3.0"]
    SIR170 = _eLeapDevicePID_eLeapDevicePID_SIR170,
    #[doc = " The Ultraleap 3Di hand tracking camera. @since 5.3.0"]
    Leap3Di = _eLeapDevicePID_eLeapDevicePID_3Di,
    #[doc = " An invalid device type. Not currently in use. @since 3.1.3"]
    Invalid = _eLeapDevicePID_eLeapDevicePID_Invalid,
}
