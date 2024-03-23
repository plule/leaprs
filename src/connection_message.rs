use derive_deref::Deref;
use leap_sys::LEAP_CONNECTION_MESSAGE;

use crate::event::EventRef;

#[doc = " Defines a basic message from the LeapC message queue."]
#[doc = " Set by calling LeapPollConnection()."]
#[doc = " @since 3.0.0"]
#[derive(Deref)]
pub struct ConnectionMessage(pub(crate) LEAP_CONNECTION_MESSAGE);

impl ConnectionMessage {
    #[doc = " A pointer to the event data for the current type of message. @since 3.0.0"]
    pub fn event(&self) -> EventRef {
        (self.type_, &self.__bindgen_anon_1).into()
    }
}
