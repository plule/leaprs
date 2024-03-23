use derive_deref::Deref;
use leap_sys::LEAP_CONFIG_CHANGE_EVENT;

#[doc = " The result of a configuration change request. Contains a status of true for a"]
#[doc = " successful change."]
#[doc = " Call LeapSaveConfigValue() to request a service config change. The change is"]
#[doc = " performed asynchronously -- and may fail. LeapPollConnection()"]
#[doc = " returns this event structure when the request has been processed. Use the requestID"]
#[doc = " value to correlate the response to the originating request."]
#[doc = " @returns The operation result code, a member of the eLeapRS enumeration."]
#[doc = " @since 3.0.0"]
#[derive(Deref)]
pub struct ConfigChangeEvent<'a>(pub(crate) &'a LEAP_CONFIG_CHANGE_EVENT);
