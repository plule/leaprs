use std::ffi::CStr;

use derive_deref::Deref;
use leap_sys::LEAP_LOG_EVENT;

use crate::LogSeverity;

#[doc = " A system log message. @since 3.0.0"]
#[derive(Deref, Clone, Copy)]
pub struct LogEventRef<'a>(pub(crate) &'a LEAP_LOG_EVENT);

impl<'a> LogEventRef<'a> {
    #[doc = " The type of message. @since 4.0.0"]
    pub fn severity(&self) -> LogSeverity {
        self.severity.into()
    }

    #[doc = " A pointer to a null-terminated string containing the current log message."]
    #[doc = " @since 4.0.0"]
    pub fn message(&self) -> Result<&str, std::str::Utf8Error> {
        let cstr = unsafe { CStr::from_ptr(self.message) };
        cstr.to_str()
    }
}
