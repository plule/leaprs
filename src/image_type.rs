use leap_sys::{
    _eLeapImageType_eLeapImageType_Default, _eLeapImageType_eLeapImageType_Raw,
    _eLeapImageType_eLeapImageType_UNKNOWN,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum ImageType {
    #[num_enum(default)]
    Unknown = _eLeapImageType_eLeapImageType_UNKNOWN,
    Default = _eLeapImageType_eLeapImageType_Default,
    Raw = _eLeapImageType_eLeapImageType_Raw,
}
