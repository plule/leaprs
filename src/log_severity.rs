use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
#[doc = " System message severity types. @since 3.0.0"]
pub enum LogSeverity {
    #[num_enum(default)]
    #[doc = " The message severity is not known or was not specified. @since 3.0.0"]
    Unknown = _eLeapLogSeverity_eLeapLogSeverity_Unknown,
    #[doc = " A message about a fault that could render the software or device non-functional. @since 3.0.0"]
    Critical = _eLeapLogSeverity_eLeapLogSeverity_Critical,
    #[doc = " A message warning about a condition that could degrade device capabilities. @since 3.0.0"]
    Warning = _eLeapLogSeverity_eLeapLogSeverity_Warning,
    #[doc = " A system status message. @since 3.0.0"]
    Information = _eLeapLogSeverity_eLeapLogSeverity_Information,
}
