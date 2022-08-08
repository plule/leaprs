use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Clone, Copy, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
#[doc = " Camera perspective types."]
#[doc = " @since 3.0.0"]
pub enum PerspectiveType {
    #[doc = " An unknown or invalid type. @since 3.0.0"]
    #[num_enum(default)]
    Invalid = _eLeapPerspectiveType_eLeapPerspectiveType_invalid,
    #[doc = " A canonically left image. @since 3.0.0"]
    StereoLeft = _eLeapPerspectiveType_eLeapPerspectiveType_stereo_left,
    #[doc = " A canonically right image. @since 3.0.0"]
    StereoRight = _eLeapPerspectiveType_eLeapPerspectiveType_stereo_right,
    #[doc = " Reserved for future use. @since 3.0.0"]
    Mono = _eLeapPerspectiveType_eLeapPerspectiveType_mono,
}
