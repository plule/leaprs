use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum ImageFormat {
    #[num_enum(default)]
    Unknown = _eLeapImageFormat_eLeapImageFormat_UNKNOWN,
    IR = _eLeapImageFormat_eLeapImageFormat_IR,
    RGBIrBayer = _eLeapImageFormat_eLeapImageFormat_RGBIr_Bayer,
}
