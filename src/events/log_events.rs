use leap_sys::LEAP_LOG_EVENTS;

use crate::LogEvent;

crate::leap_ref_struct!(
    #[doc = " A notification that a device's status has changed. One of these messages is received by the client"]
    #[doc = " as soon as the service is connected, or when a new device is attached."]
    #[doc = " @since 3.1.3"]
    LogEvents,
    LEAP_LOG_EVENTS
);

impl<'a> LogEvents<'a> {
    #[doc = " An array of ``nEvent`` LEAP_LOG_EVENT structures."]
    #[doc = " @since 4.0.0"]
    pub fn events(&self) -> Vec<LogEvent> {
        let events;
        unsafe {
            events = (0..self.handle.nEvents)
                .map(|index| (&*self.handle.events.add(index as usize)).into());
        }
        events.collect()
    }
}
