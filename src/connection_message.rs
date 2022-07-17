use leap_sys::LEAP_CONNECTION_MESSAGE;

use crate::event::Event;

crate::leap_struct!(
    #[doc = " Defines a basic message from the LeapC message queue."]
    #[doc = " Set by calling LeapPollConnection()."]
    #[doc = " @since 3.0.0"]
    ConnectionMessage,
    LEAP_CONNECTION_MESSAGE
);

impl ConnectionMessage {
    #[doc = " A unique ID for the attached device that sent this message. A value of"]
    #[doc = "  0 indicates that it was a system-wide message, and not device specific."]
    #[doc = "  Use this ID to distinguish messages sent from multiple attached devices."]
    #[doc = "  @since 4.1.0"]
    pub fn device_id(&self) -> u32 {
        self.handle.device_id
    }

    #[doc = " A pointer to the event data for the current type of message. @since 3.0.0"]
    pub fn event(&self) -> Event {
        (self.handle.type_, &self.handle.__bindgen_anon_1).into()
    }
}
