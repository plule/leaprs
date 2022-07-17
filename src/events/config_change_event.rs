use leap_sys::LEAP_CONFIG_CHANGE_EVENT;

crate::leap_ref_struct!(
    #[doc = " The result of a configuration change request. Contains a status of true for a"]
    #[doc = " successful change."]
    #[doc = " Call LeapSaveConfigValue() to request a service config change. The change is"]
    #[doc = " performed asynchronously -- and may fail. LeapPollConnection()"]
    #[doc = " returns this event structure when the request has been processed. Use the requestID"]
    #[doc = " value to correlate the response to the originating request."]
    #[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
    #[doc = " @since 3.0.0"]
    ConfigChangeEvent,
    LEAP_CONFIG_CHANGE_EVENT
);

impl<'a> ConfigChangeEvent<'a> {
    #[doc = " An identifier for correlating the request and response. @since 3.0.0"]
    pub fn request_id(&self) -> u32 {
        self.handle.requestID
    }

    #[doc = " The result of the change operation: true on success; false on failure. @since 3.0.0"]
    pub fn status(&self) -> bool {
        self.handle.status
    }
}
