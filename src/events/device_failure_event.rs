use leap_sys::LEAP_DEVICE_FAILURE_EVENT;

use crate::{Device, DeviceStatus};

crate::leap_ref_struct!(
    #[doc = " Device failure information."]
    #[doc = " LeapPollConnection() produces a message containing this event when a"]
    #[doc = " device fails. Only partial information may be available. If hDevice is"]
    #[doc = " non-null, then you can use it to identify the failed device with a known,"]
    #[doc = " open device."]
    #[doc = " @since 3.0.0"]
    DeviceFailureEvent,
    LEAP_DEVICE_FAILURE_EVENT
);

impl<'a> DeviceFailureEvent<'a> {
    #[doc = " The status of this failure event. @since 3.0.0"]
    pub fn status(&self) -> DeviceStatus {
        DeviceStatus::from_bits_truncate(self.handle.status as u32)
    }

    #[doc = " A handle to the device generating this failure event, if available, otherwise NULL."]
    #[doc = ""]
    #[doc = " You are not responsible for closing this handle."]
    #[doc = " @since 3.0.0"]
    pub fn device(&self) -> Device {
        Device::attach(self.handle.hDevice)
    }
}
