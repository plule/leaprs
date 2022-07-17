use leap_sys::LEAP_DEVICE_STATUS_CHANGE_EVENT;

use crate::{DeviceRef, DeviceStatus};

crate::leap_ref_struct!(
    #[doc = " A notification that a device's status has changed. One of these messages is received by the client"]
    #[doc = " as soon as the service is connected, or when a new device is attached."]
    #[doc = " @since 3.1.3"]
    DeviceStatusChangeEvent,
    LEAP_DEVICE_STATUS_CHANGE_EVENT
);

impl<'a> DeviceStatusChangeEvent<'a> {
    #[doc = " A reference to the device whose status has changed"]
    pub fn device(&self) -> DeviceRef {
        self.handle.device.into()
    }

    #[doc = " The last known status of the device. This is a combination of eLeapDeviceStatus flags. @since 3.1.3"]
    pub fn status(&self) -> DeviceStatus {
        DeviceStatus::from_bits_truncate(self.handle.status)
    }

    #[doc = " The current status of the device. This is a combination of eLeapDeviceStatus flags. @since 3.1.3"]
    pub fn last_status(&self) -> DeviceStatus {
        DeviceStatus::from_bits_truncate(self.handle.last_status)
    }
}
