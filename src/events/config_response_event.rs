use leap_sys::LEAP_CONFIG_RESPONSE_EVENT;

use crate::Variant;

crate::leap_ref_struct!(
    #[doc = " Contains the response to a configuration value request."]
    #[doc = " Call LeapRequestConfigValue() to request a service config value. The value is"]
    #[doc = " fetched asynchronously since it requires a service transaction. LeapPollConnection()"]
    #[doc = " returns this event structure when the request has been processed. Use the requestID"]
    #[doc = " value to correlate the response to the originating request."]
    #[doc = " @since 3.0.0"]
    ConfigResponseEvent,
    LEAP_CONFIG_RESPONSE_EVENT
);

impl<'a> ConfigResponseEvent<'a> {
    #[doc = " An identifier for correlating the request and response. @since 3.0.0"]
    pub fn request_id(&self) -> u32 {
        self.handle.requestID
    }

    #[doc = " The configuration value retrieved from the service. Do not free any memory pointed to by"]
    #[doc = " this member. The value held is only valid until the next call to LeapPollConnection()."]
    #[doc = " @since 3.0.0"]
    pub fn value(&self) -> Variant {
        self.handle.value.into()
    }
}
