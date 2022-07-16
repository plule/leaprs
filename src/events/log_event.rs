use std::ffi::CStr;

use leap_sys::LEAP_LOG_EVENT;

use crate::LogSeverity;

crate::leap_ref_struct!(LogEvent, LEAP_LOG_EVENT);

impl<'a> LogEvent<'a> {
    pub fn severity(&self) -> LogSeverity {
        self.handle.severity.into()
    }

    pub fn timestap(&self) -> i64 {
        self.handle.timestamp
    }

    pub fn message(&self) -> Result<&str, std::str::Utf8Error> {
        let cstr = unsafe { CStr::from_ptr(self.handle.message) };
        cstr.to_str()
    }
}
