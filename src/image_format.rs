use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
#[doc = " Image formats."]
#[doc = " @since 3.0.0"]
pub enum ImageFormat {
    #[num_enum(default)]
    #[doc = " An invalid or unknown format. @since 3.0.0"]
    Unknown = _eLeapImageFormat_eLeapImageFormat_UNKNOWN,
    #[doc = " An infrared image. @since 3.0.0"]
    IR = _eLeapImageFormat_eLeapImageFormat_IR,
    #[doc = " A Bayer RGBIr image with uncorrected RGB channels. @since 3.0.0"]
    RGBIrBayer = _eLeapImageFormat_eLeapImageFormat_RGBIr_Bayer,
}
