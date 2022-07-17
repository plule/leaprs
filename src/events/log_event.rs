use std::ffi::CStr;

use leap_sys::LEAP_LOG_EVENT;

use crate::LogSeverity;

crate::leap_ref_struct!(
    #[doc = " A system log message. @since 3.0.0"]
    LogEvent,
    LEAP_LOG_EVENT
);

impl<'a> LogEvent<'a> {
    #[doc = " The type of message. @since 4.0.0"]
    pub fn severity(&self) -> LogSeverity {
        self.handle.severity.into()
    }

    #[doc = " The timestamp of the message in microseconds."]
    #[doc = " Compare with the current values of LeapGetNow() and the system clock to"]
    #[doc = " calculate the absolute time of the message."]
    #[doc = " @since 4.0.0"]
    pub fn timestap(&self) -> i64 {
        self.handle.timestamp
    }

    #[doc = " A pointer to a null-terminated string containing the current log message."]
    #[doc = " @since 4.0.0"]
    pub fn message(&self) -> Result<&str, std::str::Utf8Error> {
        let cstr = unsafe { CStr::from_ptr(self.handle.message) };
        cstr.to_str()
    }
}
