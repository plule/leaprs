use leap_sys::LEAP_LOG_EVENTS;

use crate::LogEvent;

crate::leap_ref_struct!(LogEvents, LEAP_LOG_EVENTS);

impl<'a> LogEvents<'a> {
    pub fn events(&self) -> Vec<LogEvent> {
        let events;
        unsafe {
            events = (0..self.handle.nEvents)
                .into_iter()
                .map(|index| (&*self.handle.events.add(index as usize)).into());
        }
        events.collect()
    }
}
