use leap_sys::*;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Eq, PartialEq, IntoPrimitive, FromPrimitive)]
#[repr(i32)]
pub enum LogSeverity {
    #[num_enum(default)]
    Unknown = _eLeapLogSeverity_eLeapLogSeverity_Unknown,
    Critical = _eLeapLogSeverity_eLeapLogSeverity_Critical,
    Warning = _eLeapLogSeverity_eLeapLogSeverity_Warning,
    Information = _eLeapLogSeverity_eLeapLogSeverity_Information,
}
