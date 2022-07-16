use leap_sys::LEAP_CONNECTION_MESSAGE;

use crate::event::Event;

crate::leap_struct!(ConnectionMessage, LEAP_CONNECTION_MESSAGE);

impl ConnectionMessage {
    pub fn get_event(&self) -> Event {
        return (self.handle.type_, &self.handle.__bindgen_anon_1).into();
    }
}
