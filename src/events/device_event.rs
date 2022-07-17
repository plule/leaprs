use leap_sys::LEAP_DEVICE_EVENT;

use crate::{DeviceRef, DeviceStatus};

crate::leap_ref_struct!(
    #[doc = " Device event information."]
    #[doc = ""]
    #[doc = " LeapPollConnection() produces a message containing this event when a new"]
    #[doc = " device is detected. You can use the handle provided by the device filed to"]
    #[doc = " open a device so that you can access its properties."]
    #[doc = " @since 3.0.0"]
    DeviceEvent,
    LEAP_DEVICE_EVENT
);

impl<'a> DeviceEvent<'a> {
    #[doc = " The handle reference of to the newly attached device. @since 3.0.0"]
    pub fn device(&self) -> DeviceRef {
        self.handle.device.into()
    }

    #[doc = " The status of the connected device. A combination of flags from the eLeapDeviceStatus collection."]
    pub fn status(&self) -> DeviceStatus {
        DeviceStatus::from_bits_truncate(self.handle.status)
    }
}
