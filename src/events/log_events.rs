use derive_deref::Deref;
use leap_sys::LEAP_LOG_EVENTS;

use crate::LogEventRef;

#[doc = " A notification that a device's status has changed. One of these messages is received by the client"]
#[doc = " as soon as the service is connected, or when a new device is attached."]
#[doc = " @since 3.1.3"]
#[derive(Deref, Clone, Copy)]
pub struct LogEventsRef<'a>(pub(crate) &'a LEAP_LOG_EVENTS);

impl<'a> LogEventsRef<'a> {
    #[doc = " An array of ``nEvent`` LEAP_LOG_EVENT structures."]
    #[doc = " @since 4.0.0"]
    pub fn events(&self) -> Vec<LogEventRef> {
        let events;
        unsafe {
            events = (0..self.nEvents)
                .map(|index| (&*self.events.add(index as usize)))
                .map(LogEventRef);
        }
        events.collect()
    }
}
