use leap_sys::LEAP_CONNECTION_LOST_EVENT;

crate::leap_ref_struct!(
    #[doc = " Received from LeapPollConnection() when a connection to the Ultraleap Tracking Service is lost."]
    #[doc = ""]
    #[doc = " If a LeapC function that performs a transaction with the Ultraleap Tracking Service is called"]
    #[doc = " after the connection is lost, the next call to LeapPollConnection() will return"]
    #[doc = " this event. Otherwise, it can take up to 5 seconds of polling the connection to"]
    #[doc = " receive this event."]
    #[doc = " @since 3.0.0"]
    ConnectionLostEvent,
    LEAP_CONNECTION_LOST_EVENT
);

// No impl for now, only reserved fields.
