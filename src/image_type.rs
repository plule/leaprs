use leap_sys::{
    _eLeapImageType_eLeapImageType_Default, _eLeapImageType_eLeapImageType_Raw,
    _eLeapImageType_eLeapImageType_UNKNOWN,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
#[doc = " Functional image types (not data formats)."]
pub enum ImageType {
    #[num_enum(default)]
    #[doc = " An invalid or unknown type. @since 3.0.0"]
    Unknown = _eLeapImageType_eLeapImageType_UNKNOWN,
    #[doc = " Default, processed IR images. @since 3.0.0"]
    Default = _eLeapImageType_eLeapImageType_Default,
    #[doc = " Raw images from the device. @since 3.0.0"]
    Raw = _eLeapImageType_eLeapImageType_Raw,
}
